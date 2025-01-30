use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;
use log::info;
use crate::models::{Sample, LoginRequest, User};
use crate::schema::samples::dsl as samples_dsl;
use crate::schema::users::dsl as users_dsl;

mod models;
mod schema;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn db_pool() -> DbPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Server is healthy")
}

async fn add_sample(pool: web::Data<DbPool>, sample: web::Json<Sample>) -> impl Responder {
    let new_sample = sample.into_inner();
    let mut connection = pool.get().expect("Failed to get connection from the pool");

    let result = diesel::insert_into(samples_dsl::samples)
        .values((
            samples_dsl::name.eq(new_sample.name),
            samples_dsl::sample_type.eq(new_sample.sample_type),
            samples_dsl::collected_at.eq(new_sample.collected_at),
            samples_dsl::status.eq(new_sample.status),
        ))
        .get_result::<Sample>(&mut connection);

    match result {
        Ok(sample) => HttpResponse::Created().json(sample),
        Err(_) => HttpResponse::InternalServerError().body("Error adding sample"),
    }
}

async fn get_sample(pool: web::Data<DbPool>, sample_id: web::Path<i32>) -> impl Responder {
    let id = sample_id.into_inner();
    let mut connection = pool.get().expect("Failed to get connection from the pool");

    let sample = samples_dsl::samples
        .filter(samples_dsl::id.eq(id))
        .first::<Sample>(&mut connection);

    match sample {
        Ok(sample) => HttpResponse::Ok().json(sample),
        Err(_) => HttpResponse::NotFound().body("Sample not found"),
    }
}

async fn get_samples(pool: web::Data<DbPool>) -> impl Responder {
    let mut connection = pool.get().expect("Failed to get connection from the pool");

    let samples = samples_dsl::samples
        .load::<Sample>(&mut connection);

    match samples {
        Ok(samples) => HttpResponse::Ok().json(samples),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching samples"),
    }
}

async fn login(pool: web::Data<DbPool>, login: web::Json<LoginRequest>) -> impl Responder {
    let login_data = login.into_inner();
    let mut connection = pool.get().expect("Failed to get connection from the pool");

    let user = users_dsl::users
        .filter(users_dsl::username.eq(login_data.username))
        .first::<User>(&mut connection);

    match user {
        Ok(user) => {
            if user.password == login_data.password { // Ideally, compare hashed passwords
                HttpResponse::Ok().json(user)
            } else {
                HttpResponse::Unauthorized().body("Invalid credentials")
            }
        }
        Err(_) => HttpResponse::Unauthorized().body("Invalid credentials"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("🚀 Server running at http://127.0.0.1:8080");

    let pool = db_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/health", web::get().to(health_check))
            .route("/samples", web::post().to(add_sample))
            .route("/samples", web::get().to(get_samples))
            .route("/samples/{id}", web::get().to(get_sample))
            .route("/login", web::post().to(login))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
