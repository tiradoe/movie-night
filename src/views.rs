use std::collections::HashMap;

use rocket::http::CookieJar;
use rocket::serde::json::serde_json::json;
use rocket::State;
use rocket_dyn_templates::Template;
use sqlx::MySqlPool;

use crate::controllers::{movielist_controller, schedule_controller};
use crate::controllers::auth_controller::auth_check;

#[get("/")]
pub async fn login(pool: &State<MySqlPool>, cookies: &CookieJar<'_>) -> Template {
    let logged_in = match cookies.get("token").map(|crumb| crumb.value()) {
        None => false,
        Some(token) => {
            auth_check(token).await
        }
    };

    let context = HashMap::from([("", "")]);

    if logged_in {
        return admin(pool, cookies).await;
    }

    Template::render("login", &context)
}

#[get("/admin")]
pub async fn admin(pool: &State<MySqlPool>, cookies: &CookieJar<'_>) -> Template {
    match cookies.get("token").map(|crumb| crumb.value()) {
        None => {
            let context = HashMap::from([("", "")]);
            Template::render("login", &context)
        }
        Some(token) => {
            match auth_check(token).await {
                true => {
                    let lists = movielist_controller::get_lists(pool).await;
                    let context = HashMap::from([("lists", lists)]);
                    Template::render("admin", &context)
                }
                false => {
                    let context = HashMap::from([("", "")]);
                    Template::render("login", &context)
                }
            }
        }
    }
}


#[get("/lists")]
pub async fn lists(pool: &State<MySqlPool>) -> Template {
    let lists = movielist_controller::get_lists(pool).await;
    let context = HashMap::from([("lists", lists)]);

    Template::render("lists", &context)
}

#[get("/list/<id>")]
pub async fn list(pool: &State<MySqlPool>, id: u32) -> Template {
    let list = movielist_controller::get_list(pool, id).await;
    let context = HashMap::from([("list", list)]);

    Template::render("list", &context)
}

#[get("/schedules/<id>")]
pub async fn schedule(cookies: &CookieJar<'_>, pool: &State<MySqlPool>, id: u32) -> Template {
    let logged_in = match cookies.get("token").map(|crumb| crumb.value()) {
        None => false,
        Some(token) => {
            auth_check(token).await
        }
    };

    let showings = schedule_controller::get_schedule(pool, id, Some(false)).await;
    let context = HashMap::from([("showings", showings), ("logged_in", json!(logged_in))]);

    Template::render("schedule", &context)
}

#[get("/password-reset")]
pub async fn password_reset() -> Template {
    let context = HashMap::from([("", "")]);
    Template::render("password_reset", &context)
}


#[catch(404)]
pub fn not_found() -> Template {
    Template::render("404", HashMap::from([("", "")]))
}

#[catch(500)]
pub fn server_error() -> Template {
    Template::render("500", HashMap::from([("", "")]))
}
