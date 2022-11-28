use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct MovieList {
    pub id: Option<i32>,
    pub name: String,
    pub public: bool,
    pub owner: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

impl MovieList {
    pub fn new() -> Self {
        let dt = Utc::now();
        let timestamp: NaiveDateTime = dt.naive_utc();

        Self {
            id: None,
            name: String::new(),
            public: false,
            owner: 1,
            created_at: timestamp,
            updated_at: timestamp,
            deleted_at: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct MovieMovieList {
    movielist_id: u32,
    movie_id: u32,
}