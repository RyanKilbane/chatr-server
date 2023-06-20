use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct NewRoom{
    room_name: String,
    room_owner: String,
    is_private: bool,
    password: Option<String>
}
