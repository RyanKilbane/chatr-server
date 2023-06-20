mod rooms;
use rooms::NewRoom;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::time::Duration;
use sqlx::postgres::PgPoolOptions;

#[get("/v1/rooms")]
async fn get_rooms() -> impl Responder{
    HttpResponse::Ok()
}

#[post("/v1/rooms")]
async fn make_room(_new_room: web::Json<NewRoom>) -> impl Responder{
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