use std::error::Error;
use async_trait::async_trait;
use rocket::http::CookieJar;

#[async_trait]
pub trait MovieDB<T> {
    fn new() -> Self;
    async fn search(&self, term: &str, cookies: &CookieJar<'_>) -> Result<T, Box<dyn Error>>;
}