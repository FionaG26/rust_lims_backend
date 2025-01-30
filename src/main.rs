use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;
use log::info;
use actix_web::web::Json;
use bcrypt::{verify};
use chrono::NaiveDateTime;
use crate::models::{Sample, LoginRequest, User};

mod models;
mod schema;

// Type alias for the database pool
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

// Initialize the database pool using environment variables for the database URL
async fn db_pool() -> DbPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

// Health check route
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Server is healthy")
}

// Add a sample to the database
async fn add_sample(pool: web::Data<DbPool>, sample: web::Json<Sample>) -> impl Responder {
    let new_sample = sample.into_inner();

    let connection = pool.get().expect("Failed to get a connection from the pool");

    let result = diesel::insert_into(schema::samples::table)
        .values((
            schema::samples::name.eq(new_sample.name),
            schema::samples::sample_type.eq(new_sample.sample_type),
            schema::samples::collected_at.eq(new_sample.collected_at),
            schema::samples::status.eq(new_sample.status),
        ))
        .returning((
            schema::samples::id,
            schema::samples::name,
            schema::samples::sample_type,
            schema::samples::collected_at,
            schema::samples::status,
        ))
        .get_result::<Sample>(&connection);

    match result {
        Ok(sample) => HttpResponse::Created().json(sample),
        Err(_) => HttpResponse::InternalServerError().body("Error adding sample"),
    }
}

// Get samples from the database, optionally filtering by status
async fn get_samples(pool: web::Data<DbPool>, status: Option<String>) -> impl Responder {
    let connection = pool.get().expect("Failed to get a connection from the pool");

    let query = if let Some(status) = status {
        schema::samples::table
            .filter(schema::samples::status.eq(status))
            .load::<Sample>(&connection)
    } else {
        schema::samples::table
            .load::<Sample>(&connection)
    };

    match query {
        Ok(samples) => HttpResponse::Ok().json(samples),
        Err(_) => HttpResponse::InternalServerError().body("Error retrieving samples"),
    }
}

// Update sample status
async fn update_sample_status(pool: web::Data<DbPool>, id: web::Path<i32>, sample: web::Json<Sample>) -> impl Responder {
    let updated_sample = sample.into_inner();

    let connection = pool.get().expect("Failed to get a connection from the pool");

    let result = diesel::update(schema::samples::table)
        .filter(schema::samples::id.eq(*id))
        .set(schema::samples::status.eq(updated_sample.status))
        .execute(&connection);

    match result {
        Ok(_) => HttpResponse::Ok().json("Sample status updated"),
        Err(_) => HttpResponse::InternalServerError().body("Error updating sample status"),
    }
}

// Delete a sample from the database
async fn delete_sample(pool: web::Data<DbPool>, id: web::Path<i32>) -> impl Responder {
    let connection = pool.get().expect("Failed to get a connection from the pool");

    let result = diesel::delete(schema::samples::table)
        .filter(schema::samples::id.eq(*id))
        .execute(&connection);

    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Error deleting sample"),
    }
}

// Login functionality with password verification
async fn login(pool: web::Data<DbPool>, login_request: web::Json<LoginRequest>) -> impl Responder {
    use crate::schema::users::dsl::*;

    let connection = pool.get().expect("Failed to get a connection from the pool");

    let user_result = users
        .filter(username.eq(&login_request.username))
        .first::<User>(&connection);

    match user_result {
        Ok(user) => {
            if verify(&login_request.password, &user.password).unwrap_or(false) {
                info!("Login successful for user: {}", user.username);
                HttpResponse::Ok().json("Login successful")
            } else {
                info!("Invalid password for user: {}", login_request.username);
                HttpResponse::Unauthorized().json("Invalid credentials")
            }
        },
        Err(_) => {
            info!("User not found: {}", login_request.username);
            HttpResponse::NotFound().json("User not found")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("🚀 Server running at http://127.0.0.1:8080");

    let pool = db_pool().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Share the database pool
            .route("/health", web::get().to(health_check))
            .route("/samples", web::post().to(add_sample))
            .route("/samples", web::get().to(get_samples))
            .route("/samples/{id}", web::put().to(update_sample_status))
            .route("/samples/{id}", web::delete().to(delete_sample))
            .route("/login", web::post().to(login))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
