use rocket::{Request, request};
use rocket::http::{Status};
use rocket::outcome::Outcome;
use rocket::request::FromRequest;

use crate::controllers::auth_controller::auth_check;

pub struct Token(pub String);

#[derive(Debug)]
pub enum ApiTokenError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = ApiTokenError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = request.cookies().get("token")
            .map(|token| token.value().to_string( ));

        match token {
            Some(token) => {
                if auth_check(&token).await {
                    Outcome::Success(Token(token))
                } else {
                    Outcome::Failure((Status::Unauthorized, ApiTokenError::Invalid))
                }
            }
            None => Outcome::Failure((Status::Unauthorized, ApiTokenError::Missing)),
        }
    }
}