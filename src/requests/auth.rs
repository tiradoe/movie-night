use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
    pub token: Option<String>,
}