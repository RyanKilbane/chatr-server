use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::time::Duration;
use sqlx::postgres::PgPoolOptions;

#[get("/get/v1/rooms")]
async fn get_rooms() -> impl Responder{
    HttpResponse::Ok()
}

#[actix::main]
async fn main() -> std::io::Result<()>{
    const WORKERS: usize = 10;
    const DB_URL: &str = "postgres://postgres:password@localhost:5432/postgres";
    let pool = PgPoolOptions::new()
        .min_connections(10)
        .max_lifetime(Some(Duration::from_secs(3)))
        .connect(DB_URL)
        .await
        .unwrap();
    HttpServer::new(
        move || App::new().app_data(pool.clone()).service(get_rooms))
        .bind(("127.0.0.1", 8081))?
        .workers(WORKERS)
        .run()
        .await
}