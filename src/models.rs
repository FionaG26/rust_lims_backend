use chrono::{NaiveDateTime, serde::ts_seconds};
use serde::{Serialize, Deserialize};
use diesel::prelude::*;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Sample {
    #[serde(skip_deserializing)] // Skip the 'id' field during deserialization
    pub id: i32,
    pub name: String,
    pub sample_type: String,
    #[serde(with = "ts_seconds")]
    pub collected_at: NaiveDateTime,
    pub status: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Queryable, Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}
