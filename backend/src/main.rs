use actix_web::{get, web, App, HttpServer, Responder};
use sqlx::postgres::PgPool;

#[get("/api/health")]
async fn health_check() -> impl Responder {
    web::Json(serde_json::json!({ "status": "OK" }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.expect("Failed to connect to the database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(health_check)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
