use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct AccountRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub username: String,
}
