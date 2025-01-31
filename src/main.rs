use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;
use log::info;
use bcrypt::{hash, verify, DEFAULT_COST};
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

// One-time function to hash plaintext passwords in the database
async fn hash_existing_passwords(pool: web::Data<DbPool>) {
    use crate::schema::users::dsl::{users, id, password};
    let mut connection = pool.get().expect("Failed to get connection from the pool");

    let results = users
        .select((id, password))
        .load::<(i32, String)>(&mut connection)
        .expect("Error loading users");

    for (user_id, plaintext_password) in results {
        // Skip already hashed passwords
        if plaintext_password.starts_with("$2b$") {
            continue;
        }

        match hash(&plaintext_password, DEFAULT_COST) {
            Ok(hashed_password) => {
                diesel::update(users.filter(id.eq(user_id)))
                    .set(password.eq(hashed_password))
                    .execute(&mut connection)
                    .expect("Error updating password");
                println!("Password hashed for user ID: {}", user_id);
            }
            Err(err) => {
                eprintln!("Failed to hash password for user ID {}: {}", user_id, err);
            }
        }
    }
}

async fn login(pool: web::Data<DbPool>, login_request: web::Json<LoginRequest>) -> impl Responder {
    let login_data = login_request.into_inner();
    let mut connection = pool.get().expect("Failed to get connection from the pool");

    let user = users_dsl::users
        .filter(users_dsl::username.eq(login_data.username))
        .first::<User>(&mut connection);

    match user {
        Ok(user) => {
            if verify(login_data.password, &user.password).unwrap_or(false) {
                HttpResponse::Ok().json(user)
            } else {
                HttpResponse::Unauthorized().body("Invalid credentials")
            }
        }
        Err(_) => HttpResponse::Unauthorized().body("Invalid credentials"),
    }
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

    let samples = samples_dsl::samples.load::<Sample>(&mut connection);

    match samples {
        Ok(samples) => HttpResponse::Ok().json(samples),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching samples"),
    }
}

async fn delete_sample(pool: web::Data<DbPool>, sample_id: web::Path<i32>) -> impl Responder {
    let id = sample_id.into_inner();
    let mut connection = pool.get().expect("Failed to get connection from the pool");

    let result = diesel::delete(samples_dsl::samples.filter(samples_dsl::id.eq(id)))
        .execute(&mut connection);

    match result {
        Ok(0) => HttpResponse::NotFound().body("Sample not found"),
        Ok(_) => HttpResponse::Ok().body("Sample deleted"),
        Err(_) => HttpResponse::InternalServerError().body("Error deleting sample"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("🚀 Server running at http://127.0.0.1:8080");

    let pool = db_pool();

    // Run the one-time password hashing function
    hash_existing_passwords(web::Data::new(pool.clone())).await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/health", web::get().to(health_check))
            .route("/login", web::post().to(login))
            .route("/samples", web::post().to(add_sample))
            .route("/samples", web::get().to(get_samples))
            .route("/samples/{id}", web::get().to(get_sample))
            .route("/samples/{id}", web::delete().to(delete_sample))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
