use serde_derive::{Deserialize, Serialize};

use crate::{connection::Connection, common::MoveType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub room_id: i32,
    pub connections: Vec<Connection>,
    pub players:Vec<RoomUser>
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomUser {
    pub player_id:i32,
    pub selected_move:Option<MoveType>
}
