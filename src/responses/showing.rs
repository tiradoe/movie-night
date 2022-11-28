use chrono::NaiveDateTime;
use rocket::serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct ShowingResponse {
    pub id: i32,
    pub title: String,
    pub poster: String,
    pub plot: String,
    pub showtime: NaiveDateTime,
}
