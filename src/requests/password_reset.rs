use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PasswordResetRequest {
    pub email: String,
    pub current_password: String,
    pub new_password: String,
}