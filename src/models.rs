use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use diesel::{Queryable, Insertable};
use crate::schema::samples;
use crate::schema::users;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "samples"]
pub struct Sample {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub name: String,
    pub sample_type: String,
    pub collected_at: NaiveDateTime,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String, // Store the hashed password
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}
