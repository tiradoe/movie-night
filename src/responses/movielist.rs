use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};
use crate::models::movie::Movie;
use crate::models::movielist::MovieList;

#[derive(Serialize, Deserialize)]
pub struct MovieListResponse {
    pub list: MovieList,
    pub movies: Vec<Movie>,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct MovieListsResponse {
    pub id: i32,
    pub name: String,
    pub public: bool,
    pub owner: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
    pub movie_count: i32,
}
