use rocket::serde::Serialize;
use serde::Deserialize;
use crate::models::omdb::OMDbSearchMovie;

#[derive(Serialize, Deserialize)]
pub struct OMDbSearchResponse {
    #[serde(rename = "Search")]
    pub results: Vec<OMDbSearchMovie>,
}
