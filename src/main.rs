use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use log::info;
use rand::random;
use env_logger;

mod models;
use models::{Sample, LoginRequest};

// Enable logging using env_logger
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("🚀 Server running at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .route("/samples", web::post().to(add_sample))
            .route("/samples/{id}", web::put().to(update_sample_status))
            .route("/samples/{id}", web::delete().to(delete_sample))
            .route("/login", web::post().to(login))
            .route("/health", web::get().to(health_check))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// Sample model
#[derive(Debug, Serialize, Deserialize)]
pub struct Sample {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,  // Make id optional for the POST request
    pub name: String,
    pub sample_type: String,
    pub collected_at: NaiveDateTime,
    pub status: String,
}

// LoginRequest model
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

// Route handlers
async fn add_sample(sample: web::Json<Sample>) -> impl Responder {
    let mut new_sample = sample.into_inner();

    // Generate a new id (could be based on a DB sequence or UUID)
    new_sample.id = Some(random::<i32>());  // Random id generation for demo

    info!("Adding sample: {:?}", new_sample);

    HttpResponse::Created().json(new_sample)  // Return the created sample with the id
}

async fn update_sample_status(id: web::Path<i32>, sample: web::Json<Sample>) -> impl Responder {
    info!("Updating sample status for ID: {} - {:?}", id, sample);
    HttpResponse::Ok().json("Sample status updated")
}

async fn delete_sample(id: web::Path<i32>) -> impl Responder {
    info!("Deleting sample with ID: {}", id);
    HttpResponse::NoContent().finish()
}

async fn login(login_request: web::Json<LoginRequest>) -> impl Responder {
    info!("Login attempt for user: {}", login_request.username);
    HttpResponse::Ok().json("Login successful")
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().json("Server is healthy")
}

