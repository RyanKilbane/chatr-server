mod rooms;
mod persist_message;

use command_reader::lexer::{Lexer, Tokens};
use message::client_server;
use persist_message::save_message;
use rooms::{make_new_room, get_all_rooms, NewRoom};
use actix_web::{get, post, App, HttpResponse, HttpRequest, HttpServer, Responder};
use std::time::Duration;
use sqlx::postgres::PgPoolOptions;


async fn get_rooms(app_data: HttpRequest) -> impl Responder{
    let pool = app_data.app_data::<sqlx::Pool<sqlx::Postgres>>().unwrap();
    get_all_rooms(pool).await;
    HttpResponse::Ok()
}

async fn make_room<'a>(new_room: &Vec<Tokens<'a>>, app_data: HttpRequest) -> impl Responder{
    let con = app_data.app_data::<sqlx::Pool<sqlx::Postgres>>().unwrap();
    let room_name = new_room.get(1).unwrap();
    let room_name = match room_name{
        Tokens::Arg(val) => {
            let x = String::from(val.clone());
            x
        }
        _ => return HttpResponse::BadRequest()
    };

    let new_room = NewRoom{room_name: room_name, ..Default::default()};
    println!("{:?}", new_room);
    match make_new_room(new_room, con).await{
        Ok(_) => HttpResponse::Created(),
        Err(_e) => HttpResponse::InternalServerError()
    }
}
 
#[post("/v1/message/command")]
async fn recieve_command_message(message: String, app_data: HttpRequest) -> impl Responder{
    let command: client_server::CommandMessage = match serde_json::from_str(&message){
        Ok(val) => val,
        Err(_) => return HttpResponse::BadRequest()
    };
    let command_str = command.message.command.as_ref().unwrap();
    let mut lexer = Lexer::new(command_str);
    lexer.scan();
    
    match lexer.tokens.get(0){
        Some(token) => {
            match token{
                command_reader::lexer::Tokens::Create => {
                    make_room(&lexer.tokens, app_data).await;
                }

                _ => {
                    return HttpResponse::BadRequest()
                }
            }
        }

        None => {
            return HttpResponse::BadRequest()
        }
    }
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