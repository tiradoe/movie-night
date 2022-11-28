use std::collections::HashMap;
use rocket::futures::FutureExt;

use rocket::serde::json::{Json, Value};
use rocket::serde::json::serde_json::json;
use rocket::State;
use sqlx::MySqlPool;

use crate::controllers::movie_controller;
use crate::models::movie::Movie;
use crate::models::movielist::MovieList;
use crate::requests::add_movie::AddMovieRequest;
use crate::requests::movielist::MovieListRequest;
use crate::responses::movielist::{MovieListResponse, MovieListsResponse};

#[post("/lists", format = "application/json", data = "<list_request>")]
pub async fn create_list(pool: &State<MySqlPool>, list_request: Json<MovieListRequest>) -> Value {
    let mut list = MovieList::new();
    list.name = String::from(&list_request.name);
    list.public = list_request.public;

    match sqlx::query("
        INSERT INTO movienight.movie_lists
            (name, public, owner, created_at, updated_at)
        VALUES (?,?,?,?,?)")
        .bind(list.name)
        .bind(list.public)
        .bind(list.owner)
        .bind(list.created_at)
        .bind(list.updated_at)
        .execute(&**pool)
        .await {
            Ok(result) => result,
            Err(_) => panic!("Failed to create list.")
        };

    json!(HashMap::from([
        ("message", "Created list!"),
        ("status", "200"),
    ]))
}

#[get("/lists/<id>")]
pub async fn get_list(pool: &State<MySqlPool>, id: u32) -> Value {
    let list = match sqlx::query_as::<_, MovieList>("SELECT * FROM movie_lists WHERE id=?")
        .bind(id)
        .fetch_one(&**pool)
        .await {
            Ok(result) => result,
            Err(_) => panic!("List not found.")
        };

    let empty_movies: Vec<Movie> = Vec::new();

    let movies = match sqlx::query_as::<_, Movie>("
        SELECT *
        FROM movienight.movies
        WHERE id IN (
            SELECT movie_id from movielist_movie
            WHERE movielist_id = ?
        )
        ORDER BY title"
    )
        .bind(id)
        .fetch_all(&**pool)
        .await {
            Ok(result) => result,
            Err(_) => empty_movies
        };

    let response = MovieListResponse {
        list,
        movies,
    };

    json!(response)
}

#[get("/lists")]
pub async fn get_lists(pool: &State<MySqlPool>) -> Value {
    let empty_lists: Vec<MovieListsResponse> = Vec::new();

    let lists = match sqlx::query_as::<_, MovieListsResponse>("
            SELECT *,
                (SELECT count(*)
                FROM movielist_movie mm
                WHERE mm.movielist_id = ml.id) as movie_count
            FROM movie_lists ml
        ")
        .fetch_all(&**pool)
        .await {
            Ok(result) => result,
            Err(_) => empty_lists,
        };

    json!(lists)
}

#[post("/lists/movie", format = "application/json", data = "<add_movie_request>")]
pub async fn add_movie(pool: &State<MySqlPool>, add_movie_request: Json<AddMovieRequest>) -> Value {
    let _movie:Result<(), ()> = match sqlx::query_as::<_, Movie>("SELECT * FROM movies WHERE imdb_id=?")
        .bind(&add_movie_request.imdb_id)
        .fetch_one(&**pool)
        .await {
            Ok(movie) => {
                add_movie_to_list(pool, movie, add_movie_request.list_id)
                .await;

                Ok(())
            },
            Err(_) => {
                movie_controller::add_movie(pool, &add_movie_request.imdb_id)
                    .then(|movie| add_movie_to_list(pool, movie, add_movie_request.list_id))
                    .await;
                Ok(())
            }
    };

    json!(HashMap::from([
        ("message", "Added movie to list."),
        ("status", "200"),
    ]))
}

async fn add_movie_to_list(pool: &State<MySqlPool>, movie: Movie, list_id: u32) {
    match sqlx::query("
            INSERT INTO movienight.movielist_movie
            (movielist_id, movie_id)
            VALUES (?,?)"
    )
        .bind(list_id)
        .bind(movie.id)
        .execute(&**pool)
        .await {
            Ok(result) => result,
            Err(_) => panic!("Failed to add movie to list.")
        };
}