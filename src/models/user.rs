use rocket::serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    pub password: String,
    pub username: String,
}