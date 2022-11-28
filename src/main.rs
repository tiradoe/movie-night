extern crate core;
#[macro_use]
extern crate rocket;


use dotenv_codegen::dotenv;
use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::Template;
use sqlx::mysql::MySqlPoolOptions;

use controllers::{
    auth_controller,
    movie_controller,
    movielist_controller,
    schedule_controller,
};

mod movie_builder;
mod views;
mod models;
mod requests;
mod responses;
mod controllers;
mod traits;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let connection_url = &format!("{}:{}/{}",
                                  dotenv!("DATABASE_URL"),
                                  dotenv!("DATABASE_PORT"),
                                  dotenv!("DATABASE_NAME"),
    );

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(connection_url).await.expect("Error connecting to database");

    let _rocket = rocket::build()
        .mount("/", routes![views::list])
        .mount("/", routes![views::lists])
        .mount("/", routes![views::login])
        .mount("/", routes![views::admin])
        .mount("/", routes![views::password_reset])
        .mount("/", routes![views::schedule])
        .register("/", catchers![views::not_found])
        .register("/", catchers![views::server_error])
        .mount("/api", routes![movie_controller::get_movie])
        .mount("/api", routes![movie_controller::get_movies])
        .mount("/api", routes![movie_controller::remove_movie])
        .mount("/api", routes![movielist_controller::create_list])
        .mount("/api", routes![movielist_controller::get_list])
        .mount("/api", routes![movielist_controller::get_lists])
        .mount("/api", routes![movielist_controller::add_movie])
        .mount("/api", routes![schedule_controller::create_schedule])
        .mount("/api", routes![schedule_controller::add_movie])
        .mount("/api", routes![schedule_controller::get_schedule])
        .mount("/api", routes![schedule_controller::delete_showing])
        .mount("/api", routes![auth_controller::login])
        .mount("/api", routes![auth_controller::logout])
        .mount("/api", routes![auth_controller::create_account])
        .mount("/api", routes![auth_controller::reset_password])
        .mount("/api", routes![movie_controller::search])
        .mount("/static", FileServer::from(relative!("static")))
        .attach(Template::fairing())
        .manage(pool)
        .launch()
        .await?;
    Ok(())
}
