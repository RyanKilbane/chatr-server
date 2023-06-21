mod rooms;
mod persist_message;
use message::client_server;
use persist_message::save_message;
use rooms::{NewRoom, make_new_room, get_all_rooms};
use actix_web::{get, post, web::{self, Path}, App, HttpResponse, HttpRequest, HttpServer, Responder};
use std::time::Duration;
use sqlx::postgres::PgPoolOptions;


async fn get_rooms(app_data: HttpRequest) -> impl Responder{
    let pool = app_data.app_data::<sqlx::Pool<sqlx::Postgres>>().unwrap();
    get_all_rooms(pool).await;
    HttpResponse::Ok()
}

async fn make_room(new_room: web::Json<NewRoom>, app_data: HttpRequest) -> impl Responder{
    let con = app_data.app_data::<sqlx::Pool<sqlx::Postgres>>().unwrap();
    match make_new_room(new_room.0, con).await{
        Ok(_) => HttpResponse::Created(),
        Err(_e) => HttpResponse::InternalServerError()
    }
}

#[post("/v1/message/command")]
async fn recieve_command_message(message: String) -> impl Responder{
    let command: client_server::CommandMessage = match serde_json::from_str(&message){
        Ok(val) => val,
        Err(_) => return HttpResponse::BadRequest()
    };
    
    HttpResponse::Ok()
}

#[post("/v1/message")]
async fn recieve_message(message: String, app_data: HttpRequest) -> impl Responder{
    let message: client_server::NormalMessage = match serde_json::from_str(&message){
        Ok(val) => val,
        Err(_) => return HttpResponse::BadRequest()
    };
    let pool = app_data.app_data::<sqlx::Pool<sqlx::Postgres>>().unwrap();
    save_message(message, pool).await;
    HttpResponse::Ok()
}

#[get("/")]
async fn connected() ->impl Responder{
    println!("Connected");
    HttpResponse::Ok()
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
        move || App::new().app_data(pool.clone()).service(recieve_command_message).service(recieve_message).service(connected))
        .bind(("127.0.0.1", 8081))?
        .workers(WORKERS)
        .run()
        .await
}