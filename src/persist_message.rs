use message::client_server::NormalMessage;
use sqlx::{Pool, Postgres};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageModel{
    pub id: i32,
    pub room: String,
    pub room_owner: String,
    pub body: String
}

pub async fn save_message(new_message: NormalMessage, pool: &Pool<Postgres>) {
    let message = new_message.message;
    sqlx::query!(r#"INSERT INTO messages(time_created, room, body, sent_by) VALUES ($1, $2, $3, $4)"#, 
    new_message.time_sent.naive_utc(),
    1,
    message.message_body,
    message.sender).execute(pool).await.unwrap();
}
