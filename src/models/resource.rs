use uuid::Uuid;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};


#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Resource {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub available: bool,
    pub owner_id: i32, // Assuming owner_id is a Uuid in the database
    pub created_at: Option<NaiveDateTime>,
}
