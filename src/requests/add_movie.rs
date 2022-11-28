use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AddMovieRequest {
    pub imdb_id: String,
    pub list_id: u32,
}
