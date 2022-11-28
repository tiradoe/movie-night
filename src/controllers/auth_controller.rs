use std::collections::HashMap;
use std::ops::Add;

use argon2::Argon2;
use chrono::{Duration, NaiveDateTime, Utc};
use dotenv_codegen::dotenv;
use password_hash::{PasswordHash, PasswordVerifier};
use rand::distributions::{Alphanumeric, DistString};
use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::serde::json::{Json, Value};
use rocket::serde::json::serde_json::json;
use rocket::{State, time};
use sqlx::{MySqlConnection, MySqlPool};

use crate::models::session::Session;
use crate::models::user::User;
use crate::requests::account::AccountRequest;
use crate::requests::auth::AuthRequest;
use crate::requests::password_reset::PasswordResetRequest;

#[post("/auth/login", format = "application/json", data = "<auth_request>")]
pub async fn login(pool: &State<MySqlPool>, auth_request: Json<AuthRequest>, cookies: &CookieJar<'_>) -> Value {
    let user = match sqlx::query_as::<_, User>("
        SELECT *
        FROM users
        WHERE username=?
    ")
        .bind(&auth_request.username)
        .fetch_one(&**pool)
        .await {
        Ok(user) => {
            check_password(&user.password, &auth_request.password);
            user
        }
        Err(_) => panic!("Login failed.")
    };

    let token = update_token(pool, user).await;

    let token_cookie = Cookie::build("token", token)
        .max_age(time::Duration::days(1))
        .same_site(SameSite::Strict)
        .finish();

    cookies.add(token_cookie);

    json!(HashMap::from([
        ("message","Logged in successfully."),
        ("status","200"),
    ]))
}

#[put("/logout")]
pub async fn logout(pool: &State<MySqlPool>, cookies: &CookieJar<'_>) -> Value {
    match cookies.get("token").map(|crumb| crumb.value()) {
        Some(token) => {
            cookies.remove(Cookie::named("token"));

            sqlx::query("DELETE FROM sessions WHERE token = ?")
                .bind(token)
                .execute(&**pool)
                .await
                .expect("Failed to update token.");
        }
        None => {
            panic!("Logout failed.")
        }
    }
    json!(HashMap::from([("message", "Logged out successfully.")]))
}

#[post("/auth", format = "application/json", data = "<account_request>")]
pub async fn create_account(pool: &State<MySqlPool>, account_request: Json<AccountRequest>) -> Value {
    let salt_string = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

    let hash = PasswordHash::generate(
        Argon2::default(),
        &account_request.password,
        &salt_string,
    ).expect("Error while creating password");

    match sqlx::query("
        INSERT INTO movienight.users
            (name, email, password, username)
        VALUES (?,?,?,?)")
        .bind(&account_request.name)
        .bind(&account_request.email)
        .bind(hash.to_string())
        .bind(&account_request.username)
        .execute(&**pool)
        .await {
        Ok(result) => result,
        Err(_) => panic!("Failed to create account")
    };
    json!("Account Created")
}

async fn update_token(pool: &State<MySqlPool>, user: User) ->  String {
    let token = Alphanumeric.sample_string(&mut rand::thread_rng(), 40);

    let dt = Utc::now();
    let expiration = dt.add(Duration::hours(24));
    let token_expiration: NaiveDateTime = expiration.naive_utc();

    sqlx::query("
        INSERT INTO movienight.sessions
        (user_id, token, expiration)
        VALUES (?,?,?)
    ")
        .bind(user.id)
        .bind(&token)
        .bind(token_expiration)
        .execute(&**pool)
        .await
        .expect("Failed to update token.");

    token

}


fn check_password(user_password: &str, request_password: &str) {
    let argon2: &[&dyn PasswordVerifier] = &[&Argon2::default()];
    let password_hash = PasswordHash::new(user_password)
        .expect("Login failed");

    password_hash
        .verify_password(argon2, request_password)
        .expect("Invalid username or password");
}

#[put("/auth", format = "application/json", data = "<password_reset_request>")]
pub async fn reset_password(pool: &State<MySqlPool>, password_reset_request: Json<PasswordResetRequest>) -> Value {
    match sqlx::query_as::<_, User>("
        SELECT *
        FROM users
        WHERE email=?
    ")
        .bind(&password_reset_request.email)
        .fetch_one(&**pool)
        .await {
        Ok(user) => {
            check_password(&user.password,
                           &password_reset_request.current_password);
        }
        Err(_) => panic!("Password verification failed")
    };

    let salt_string = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

    let hash = PasswordHash::generate(
        Argon2::default(),
        &password_reset_request.new_password,
        &salt_string,
    ).expect("Error while creating password");
    let hash_string = hash.to_string();

    sqlx::query("UPDATE users SET password = ? WHERE email=?")
        .bind(hash_string)
        .bind(&password_reset_request.email)
        .execute(&**pool)
        .await
        .expect("Failed to update password.");

    json!("Password updated!")
}

pub async fn auth_check(token: &str) -> bool {
    use sqlx::Connection;
    let connection_url = &format!("{}:{}/{}",
                                  dotenv!("DATABASE_URL"),
                                  dotenv!("DATABASE_PORT"),
                                  dotenv!("DATABASE_NAME"),
    );

    let mut conn = MySqlConnection::connect(connection_url).await.expect("DB Error");

    match sqlx::query_as::<_, Session>("
        SELECT *
        FROM sessions
        WHERE token=?
    ")
        .bind(token)
        .fetch_one(&mut conn)
        .await {
        Ok(_user) => true,
        Err(_) => false
    }
}
