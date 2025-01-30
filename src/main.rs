use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use log::info;
use env_logger;
use sqlx::{PgPool, query_as};
use chrono::NaiveDateTime;

mod models;
use models::{Sample, LoginRequest};

async fn db_pool() -> PgPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPool::connect(&database_url).await.unwrap()
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Server is healthy")
}

async fn add_sample(pool: web::Data<PgPool>, sample: web::Json<Sample>) -> impl Responder {
    let new_sample = sample.into_inner();

    let result = sqlx::query!(
        "INSERT INTO samples (name, sample_type, collected_at, status) VALUES ($1, $2, $3, $4) RETURNING id",
        new_sample.name,
        new_sample.sample_type,
        new_sample.collected_at,
        new_sample.status
    )
    .fetch_one(&**pool)
    .await;

    match result {
        Ok(record) => {
            HttpResponse::Created().json(Sample {
                id: Some(record.id),
                ..new_sample
            })
        }
        Err(_) => HttpResponse::InternalServerError().body("Error adding sample"),
    }
}

async fn get_samples(pool: web::Data<PgPool>, status: Option<String>) -> impl Responder {
    let query = if let Some(status) = status {
        "SELECT * FROM samples WHERE status = $1"
    } else {
        "SELECT * FROM samples"
    };

    let samples = sqlx::query_as::<_, Sample>(query)
        .bind(status)
        .fetch_all(&**pool)
        .await;

    match samples {
        Ok(samples) => HttpResponse::Ok().json(samples),
        Err(_) => HttpResponse::InternalServerError().body("Error retrieving samples"),
    }
}

async fn update_sample_status(pool: web::Data<PgPool>, id: web::Path<i32>, sample: web::Json<Sample>) -> impl Responder {
    let updated_sample = sample.into_inner();

    let result = sqlx::query!(
        "UPDATE samples SET status = $1 WHERE id = $2",
        updated_sample.status,
        *id
    )
    .execute(&**pool)
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json("Sample status updated"),
        Err(_) => HttpResponse::InternalServerError().body("Error updating sample status"),
    }
}

async fn delete_sample(pool: web::Data<PgPool>, id: web::Path<i32>) -> impl Responder {
    let result = sqlx::query!("DELETE FROM samples WHERE id = $1", *id)
        .execute(&**pool)
        .await;

    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Error deleting sample"),
    }
}

async fn login(pool: web::Data<PgPool>, login_request: web::Json<LoginRequest>) -> impl Responder {
    use bcrypt::{verify};

    let user = sqlx::query!("SELECT password FROM users WHERE username = $1", login_request.username)
        .fetch_one(&**pool)
        .await;

    match user {
        Ok(record) => {
            if verify(&login_request.password, &record.password).unwrap_or(false) {
                HttpResponse::Ok().json("Login successful")
            } else {
                HttpResponse::Unauthorized().body("Invalid credentials")
            }
        }
        Err(_) => HttpResponse::Unauthorized().body("User not found"),
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
