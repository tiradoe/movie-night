use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ScheduleRequest {
    pub name: String,
    pub is_public: bool,
    pub slug: String,
    pub owner: u32,
}