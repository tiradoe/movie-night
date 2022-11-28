use crate::movie_builder::MovieBuilder;
use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, sqlx::FromRow)]
pub struct Movie {
    pub id: Option<i32>,
    pub title: Option<String>,
    pub imdb_id: Option<String>,
    pub year: Option<u16>,
    pub critic_score: Option<String>,
    pub genre: Option<String>,
    pub director: Option<String>,
    pub actors: Option<String>,
    pub plot: Option<String>,
    pub poster: Option<String>,
    pub is_good: Option<bool>,
    pub last_watched: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

impl Movie {
    pub fn builder() -> MovieBuilder {
        MovieBuilder::new()
    }
}
