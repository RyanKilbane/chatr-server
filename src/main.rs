mod rooms;
use rooms::{NewRoom, make_new_room, get_all_rooms};
use actix_web::{get, post, web, App, HttpResponse, HttpRequest, HttpServer, Responder};
use std::time::Duration;
use sqlx::postgres::PgPoolOptions;



#[get("/v1/rooms")]
async fn get_rooms(app_data: HttpRequest) -> impl Responder{
    let pool = app_data.app_data::<sqlx::Pool<sqlx::Postgres>>().unwrap();
    get_all_rooms(pool).await;
    HttpResponse::Ok()
}

#[post("/v1/rooms")]
async fn make_room(new_room: web::Json<NewRoom>, app_data: HttpRequest) -> impl Responder{
    let con = app_data.app_data::<sqlx::Pool<sqlx::Postgres>>().unwrap();
    match make_new_room(new_room.0, con).await{
        Ok(_) => HttpResponse::Created(),
        Err(_e) => HttpResponse::InternalServerError()
    }
}

#[actix::main]
async fn main() -> std::io::Result<()>{
    dotenv::dotenv().expect("Unable to load environment variables from .env file");
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