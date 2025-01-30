use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Sample {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,  // Make id optional when posting
    pub name: String,
    pub sample_type: String,
    pub collected_at: NaiveDateTime,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}
