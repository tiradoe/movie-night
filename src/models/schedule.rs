use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime};

#[derive(Serialize, Deserialize)]
pub struct Schedule {
    pub name: String,
    pub is_public: bool,
    pub slug: String,
    pub owner: u32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}
