use serde_derive::{Deserialize, Serialize};

use crate::{connection::Connection, common::MoveType};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Room {
    pub room_id: i32,
    pub connections: Vec<Connection>,
    pub players:Vec<RoomUser>,
    pub state: RoomState,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RoomUser {
    pub player_id:i32,
    pub selected_move:Option<MoveType>
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RoomState{
    WaitingForUsers,
    WaitingForStart,
    Playing
}