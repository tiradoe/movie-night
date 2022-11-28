use std::error::Error;

use dotenv_codegen::dotenv;
use rocket::http::CookieJar;
use rocket::serde::json::serde_json;
use serde::{Deserialize, Serialize};

use crate::controllers::auth_controller::auth_check;
use crate::responses::omdb::OMDbSearchResponse;
use crate::traits::moviedb::MovieDB;

pub struct OMDb {
    pub api_url: String,
    pub api_key: String,
}

#[async_trait]
impl MovieDB<Vec<OMDbSearchMovie>> for OMDb {
    fn new() -> Self {
        OMDb {
            api_url: String::from(dotenv!("OMDB_URL")),
            api_key: String::from(dotenv!("OMDB_API_KEY")),
        }
    }

    async fn search(&self, search_term: &str, cookies: &CookieJar<'_>) -> Result<Vec<OMDbSearchMovie>, Box<dyn Error>> {
        match cookies.get("token").map(|crumb| crumb.value()) {
            Some(token) => {
                if !auth_check(token).await {
                    panic!("Not authorized")
                }
            }
            None => {
                panic!("Not authorized")
            }
        }

        let request_url = format!("{}?apikey={}&s={}&type=movie",
                                  &self.api_url,
                                  &self.api_key,
                                  search_term
        );

        let resp = reqwest::get(request_url)
            .await?
            .text()
            .await?;

        let deserialized_response: OMDbSearchResponse = serde_json::from_str(resp.as_str())
            .unwrap_or(OMDbSearchResponse { results: Vec::new() });

        Ok(deserialized_response.results)
    }
}

impl OMDb {
    pub async fn search_by_imdb(&self, title: &String) -> Result<OMDBMovie, Box<dyn Error>> {
        let request_url = format!("{}?apikey={}&i={}&type=movie",
                                  &self.api_url,
                                  &self.api_key,
                                  title
        );

        let resp = reqwest::get(request_url)
            .await?
            .text()
            .await?;

        let deserialized_response: OMDBMovie = serde_json::from_str(resp.as_str()).expect("Error reading OMDb response.");
        Ok(deserialized_response)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OMDbSearchMovie {
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "Year")]
    pub year: String,
    #[serde(rename = "imdbID")]
    pub imdb_id: String,
    #[serde(rename = "Type")]
    pub media_type: String,
    #[serde(rename = "Poster")]
    pub poster: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OMDbRating {
    #[serde(rename = "Source")]
    pub source: String,
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OMDBMovie {
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "Year")]
    pub year: String,
    #[serde(rename = "Rated")]
    pub rated: String,
    #[serde(rename = "Released")]
    pub released: String,
    #[serde(rename = "Runtime")]
    pub runtime: String,
    #[serde(rename = "Genre")]
    pub genre: String,
    #[serde(rename = "Director")]
    pub director: String,
    #[serde(rename = "Writer")]
    pub writer: String,
    #[serde(rename = "Actors")]
    pub actors: String,
    #[serde(rename = "Plot")]
    pub plot: String,
    #[serde(rename = "Language")]
    pub language: String,
    #[serde(rename = "Country")]
    pub country: String,
    #[serde(rename = "Awards")]
    pub awards: String,
    #[serde(rename = "Poster")]
    pub poster: String,
    #[serde(rename = "Ratings")]
    pub critic_score: Vec<OMDbRating>,
    #[serde(rename = "Metascore")]
    pub metascore: String,
    #[serde(rename = "imdbRating")]
    pub imdb_rating: String,
    #[serde(rename = "imdbVotes")]
    pub imdb_votes: String,
    #[serde(rename = "imdbID")]
    pub imdb_id: String,
    #[serde(rename = "Type")]
    pub media_type: String,
    #[serde(rename = "DVD")]
    pub dvd: String,
    #[serde(rename = "BoxOffice")]
    pub box_office: String,
    #[serde(rename = "Production")]
    pub production: String,
    #[serde(rename = "Website")]
    pub website: String,
    #[serde(rename = "Response")]
    pub response: String,
}
