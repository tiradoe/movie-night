use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ShowingRequest {
    pub schedule_id: u32,
    pub movie_id: u32,
    pub showtime: String,
    pub public: bool,
    pub owner: u32,
}
