use chrono::NaiveDateTime;
use rocket::serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Session  {
    pub id: Option<u32>,
    pub user_id: i32,
    pub token: String,
    pub expiration: NaiveDateTime,
}