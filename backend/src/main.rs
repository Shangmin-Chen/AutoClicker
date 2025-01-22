use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_cors::Cors; // Import CORS
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow};
use std::env;
use dotenv::dotenv;
use chrono::NaiveDateTime;

// Room struct for JSON responses
#[derive(serde::Serialize, sqlx::FromRow)]
struct Room {
    id: i32,
    name: String,
    created_at: NaiveDateTime,
}

// Request payload for creating a room
#[derive(Deserialize)]
struct CreateRoom {
    name: String,
}

// Handler for creating a room
async fn create_room(
    pool: web::Data<PgPool>,
    room: web::Json<CreateRoom>,
) -> impl Responder {
    let query = "INSERT INTO rooms (name) VALUES ($1) RETURNING id, name, created_at";

    let result = sqlx::query_as::<_, Room>(query)
        .bind(&room.name)
        .fetch_one(pool.get_ref())
        .await;

    match result {
        Ok(created_room) => HttpResponse::Ok().json(created_room),
        Err(sqlx::Error::Database(db_err)) if db_err.constraint() == Some("rooms_name_key") => {
            HttpResponse::Conflict().body(format!("Room '{}' already exists.", room.name))
        }
        Err(e) => {
            eprintln!("Failed to create room: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to create room")
        }
    }
}

// Handler for retrieving a room
async fn get_room(
    pool: web::Data<PgPool>,
    name: web::Path<String>,
) -> impl Responder {
    let query = "SELECT id, name, created_at FROM rooms WHERE name = $1";
    let result = sqlx::query_as::<_, Room>(query)
        .bind(name.as_str())
        .fetch_one(pool.get_ref())
        .await;

    match result {
        Ok(room) => HttpResponse::Ok().json(room),
        Err(e) => {
            eprintln!("Failed to retrieve room: {:?}", e);
            HttpResponse::NotFound().body("Room not found")
        }
    }
}

// Database connection setup
async fn init_db() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to the database")
}

// Main function
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load environment variables from .env
    let pool = init_db().await;

    println!("Starting server on http://127.0.0.1:8080");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin() // Allow requests from any origin
            .allow_any_method() // Allow any HTTP method
            .allow_any_header(); // Allow any headers

        App::new()
            .wrap(cors) // Add CORS middleware
            .app_data(web::Data::new(pool.clone()))
            .route("/rooms", web::post().to(create_room))
            .route("/rooms/{name}", web::get().to(get_room))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
