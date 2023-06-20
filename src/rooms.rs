use std::{error::Error, fmt};
use sqlx::{query_as, Pool, Postgres};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct RoomCreateError{
    message: String
}

impl Error for RoomCreateError{}

impl fmt::Display for RoomCreateError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}


#[derive(Debug, Deserialize, Serialize)]
pub struct NewRoom{
    room_name: String,
    room_owner: String,
    is_private: bool,
    password: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RoomModel{
    #[serde(skip)]
    pub id: i32,
    pub room_name: String,
    #[serde(skip)]
    pub room_owner: String,
    #[serde(skip)]
    pub is_private: bool,
    #[serde(skip)]
    pub password: Option<String>
}

pub async fn make_new_room(new_room: NewRoom, pool: &Pool<Postgres>) -> Result<(), RoomCreateError>{
    sqlx::query(r#"INSERT INTO users VALUES $1, $2, $3, $4"#)
    .bind(new_room.room_name)
    .bind(new_room.room_owner)
    .bind(new_room.is_private)
    .bind(new_room.password).execute(pool).await.unwrap();
    Ok(())
}

pub async fn get_all_rooms(pool: &Pool<Postgres>) -> Vec<RoomModel>{
    query_as!(RoomModel, r#"SELECT * FROM rooms WHERE is_private = $1"#, false).fetch_all(pool).await.unwrap()
}