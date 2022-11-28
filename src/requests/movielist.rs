use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MovieListRequest {
    pub name: String,
    pub public: bool,
}