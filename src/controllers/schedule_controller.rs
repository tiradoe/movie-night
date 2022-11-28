use std::collections::HashMap;

use chrono::{Datelike, NaiveDate, NaiveDateTime, Utc};
use rocket::serde::json::{Json, Value};
use rocket::serde::json::serde_json::json;
use rocket::State;
use sqlx::MySqlPool;

use crate::models::auth::Token;
use crate::models::schedule::Schedule;
use crate::requests::schedule::ScheduleRequest;
use crate::requests::showing::ShowingRequest;
use crate::responses::showing::ShowingResponse;

#[get("/schedules/<id>?<previous>", format = "application/json")]
pub async fn get_schedule(pool: &State<MySqlPool>, id: u32, previous: Option<bool>) -> Value {
    let empty_showings: Vec<ShowingResponse> = Vec::new();
    let dt = Utc::now();

    // Get from start of day so that today's movie is included
    let timestamp = NaiveDate::from_ymd(
        dt.year(),
        dt.month(),
        dt.day())
        .and_hms(00, 00, 00);

    let mut showtime_sql = "AND showings.showtime >= ?";
    if previous.unwrap_or(false) {
        showtime_sql = "AND showings.showtime <= ?"
    }

    let showings = match sqlx::query_as::<_, ShowingResponse>(
        format!("
        SELECT movies.title, movies.poster, movies.plot, showings.id, showings.showtime
        FROM movienight.movies
        INNER JOIN movienight.showings
        ON showings.movie_id=movies.id
        WHERE showings.schedule_id >= ?
        {}
        ORDER BY showings.showtime;
    ", showtime_sql).as_str(),
    )
        .bind(id)
        .bind(timestamp)
        .fetch_all(&**pool)
        .await {
        Ok(result) => result,
        Err(_) => empty_showings,
    };

    json!(showings)
}

#[post("/schedules", format = "application/json", data = "<data>")]
pub async fn create_schedule(pool: &State<MySqlPool>, data: Json<ScheduleRequest>) -> Value {
    let dt = Utc::now();
    let timestamp = dt.naive_utc();

    let schedule = Schedule {
        name: data.name.clone(),
        is_public: data.is_public,
        slug: data.slug.clone(),
        owner: data.owner,
        created_at: timestamp,
        updated_at: timestamp,
        deleted_at: None,
    };

    match sqlx::query(
        "
        INSERT INTO movienight.schedules
            (name, is_public, slug, owner, created_at, updated_at)
        VALUES (?,?,?,?,?,?)",
    )
        .bind(schedule.name)
        .bind(schedule.is_public)
        .bind(schedule.slug)
        .bind(schedule.owner)
        .bind(schedule.created_at)
        .bind(schedule.updated_at)
        .execute(&**pool)
        .await {
        Ok(result) => result,
        Err(_) => panic!("Failed to create schedule"),
    };
    json!(HashMap::from([("message", "Created Schedule")]))
}

#[post("/schedules/movie", format = "application/json", data = "<showing_request>")]
pub async fn add_movie(_token: Token, pool: &State<MySqlPool>, showing_request: Json<ShowingRequest>) -> Value {
    let timestamp: NaiveDateTime =
        NaiveDateTime::parse_from_str(&showing_request.showtime, "%Y-%m-%d %H:%M:%S").expect("Failed to parse date.");

    match sqlx::query(
        "
        INSERT INTO movienight.showings
            (schedule_id, movie_id, showtime, public, created_at, updated_at, owner)
        VALUES (?,?,?,?,?,?,?)
        ",
    )
        .bind(&showing_request.schedule_id)
        .bind(&showing_request.movie_id)
        .bind(&showing_request.showtime)
        .bind(&showing_request.public)
        .bind(&timestamp)
        .bind(&timestamp)
        .bind(&showing_request.owner)
        .execute(&**pool)
        .await {
        Ok(result) => result,
        Err(_) => panic!("Failed to add movie to schedule."),
    };

    json!(HashMap::from([("message", "Added movie to schedule")]))
}

#[delete("/schedules/<showing_id>")]
pub async fn delete_showing(_token: Token, pool: &State<MySqlPool>, showing_id: u32) -> Value {
    sqlx::query("DELETE FROM showings WHERE id = ?")
        .bind(showing_id)
        .execute(&**pool)
        .await
        .expect("Failed to delete showing");

    json!(HashMap::from([
        ("message", "Showing Deleted"),
        ("status", "200")
    ]))
}
