use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use log::info;
use rand::random;
use env_logger;

mod models;
use models::{Sample, LoginRequest};

// Define route handlers before `main()`
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Server is healthy")
}

async fn add_sample(sample: web::Json<Sample>) -> impl Responder {
    let mut new_sample = sample.into_inner();
    new_sample.id = Some(random::<i32>());
    
    info!("Adding sample: {:?}", new_sample);
    HttpResponse::Created().json(new_sample)
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("🚀 Server running at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check)) // Ensure the health check route exists
            .route("/samples", web::post().to(add_sample))
            .route("/samples/{id}", web::put().to(update_sample_status))
            .route("/samples/{id}", web::delete().to(delete_sample))
            .route("/login", web::post().to(login))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

