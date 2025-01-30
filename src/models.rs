use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use diesel::{Queryable, Insertable};
use crate::schema::{samples, users}; // Add this to bring in the schema.

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = samples)] // Use the correct Diesel syntax.
pub struct Sample {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>, // id is optional because it may not be assigned during insert.
    pub name: String,
    pub sample_type: String,
    pub collected_at: NaiveDateTime,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = users)] // Use the correct Diesel syntax.
pub struct User {
    pub id: i32, // id is required as it's the primary key.
    pub username: String,
    pub password: String, // Store the hashed password.
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}
