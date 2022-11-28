use std::collections::HashMap;

use chrono::{NaiveDateTime, Utc};
use rocket::http::CookieJar;
use rocket::serde::json::{serde_json, Value};
use rocket::serde::json::serde_json::json;
use rocket::serde::Serialize;
use rocket::State;
use serde::Deserialize;
use sqlx::MySqlPool;
use crate::models::auth::Token;

use crate::models::movie::Movie;
use crate::models::omdb::OMDb;
use crate::traits::moviedb::MovieDB;

#[derive(Serialize, Deserialize)]
pub struct MovieRequest {
    pub imdb_id: String,
}

#[get("/movies")]
pub async fn get_movies(pool: &State<MySqlPool>) -> Value {
    let movies = sqlx::query_as::<_, Movie>("SELECT * FROM movies")
        .fetch_all(&**pool)
        .await
        .expect("Error occurred while fetching movies.");

    json!(movies)
}

#[get("/movies/<id>")]
pub async fn get_movie(pool: &State<MySqlPool>, id: u32) -> Value {
    let movie = sqlx::query_as::<_, Movie>("SELECT * FROM movies WHERE id=?")
        .bind(id)
        .fetch_one(&**pool)
        .await
        .expect("Unable to find movie");

    json!(movie)
}

#[get("/movies/search/<term>")]
pub async fn search(_token: Token, term: &str, cookies: &CookieJar<'_>) -> Value {
    let omdb = OMDb::new();
    let found = match omdb.search(term, cookies).await {
        Ok(movies) => movies,
        Err(_) => Vec::new()
    };

    json!(found)
}

#[delete("/movies/l/<list_id>/m/<movie_id>")]
pub async fn remove_movie(_token: Token, pool: &State<MySqlPool>, list_id: u32, movie_id: u32) -> Value {
    sqlx::query("DELETE FROM movielist_movie WHERE movielist_id = ? AND movie_id = ? ")
        .bind(list_id)
        .bind(movie_id)
        .execute(&**pool)
        .await
        .expect("Failed to delete movie");

    json!(HashMap::from([
        ("message", "Movie Deleted"),
        ("status", "200")
    ]))
}

/**
 * OMDb search doesn't return all of the movie fields so we need to get those
 * before inserting to the database
 */
async fn update_movie(imdb_id: &String) -> Movie {
    let omdb = OMDb::new();
    let result = omdb.search_by_imdb(imdb_id).await.expect("Failed to find movie");

    let dt = Utc::now();
    let timestamp: NaiveDateTime = dt.naive_utc();
    let critic_scores = result.critic_score;
    let mut hashed_scores = HashMap::new();

    for score in critic_scores {
        hashed_scores.insert("Score", score.source);
        hashed_scores.insert("Value", score.value);
    }
    let score_json = serde_json::to_string(&hashed_scores).unwrap();

    Movie::builder()
        .actors(result.actors)
        .created_at(timestamp)
        .critic_score(score_json)
        .director(result.director)
        .genre(result.genre)
        .imdb_id(result.imdb_id)
        .plot(result.plot)
        .poster(result.poster)
        .title(result.title)
        .year(result.year.parse::<u16>().unwrap())
        .updated_at(timestamp)
        .build()
}

pub async fn add_movie(pool: &State<MySqlPool>, imdb_id: &String) -> Movie {
    let movie: Movie;

    let existing_movie = match sqlx::query_as::<_, Movie>("SELECT * FROM movies WHERE imdb_id=?")
        .bind(imdb_id)
        .fetch_one(&**pool)
        .await {
            Ok(movie) => Ok(movie),
            Err(_) => Err("No existing movie")
        };

    if let Ok(temp_movie) = existing_movie {
        movie = temp_movie;
    } else {
        movie = update_movie(imdb_id).await;
    }

    match sqlx::query("
        INSERT INTO movienight.movies
            (title, imdb_id, year, critic_score, genre, director, actors,
            plot, poster, last_watched, created_at, updated_at, deleted_at)
        VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?)")
        .bind(&movie.title)
        .bind(&movie.imdb_id)
        .bind(&movie.year)
        .bind(&movie.critic_score)
        .bind(&movie.genre)
        .bind(&movie.director)
        .bind(&movie.actors)
        .bind(&movie.plot)
        .bind(&movie.poster)
        .bind(&movie.last_watched)
        .bind(&movie.created_at)
        .bind(&movie.updated_at)
        .bind(&movie.deleted_at)
        .execute(&**pool)
        .await {
            Ok(result) => result,
            Err(_) => panic!("Failed to add movie.")
        };

    match sqlx::query_as::<_, Movie>("SELECT * FROM movienight.movies WHERE imdb_id=?")
        .bind(movie.imdb_id)
        .fetch_one(&**pool)
        .await {
            Ok(movie) => movie,
            Err(_) => panic!("Failed to find movie."),
        }
}