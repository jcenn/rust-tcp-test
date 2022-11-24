use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct JoinRoomRequest{
    pub room_id:i32,
}

#[derive(Serialize, Deserialize)]
pub struct RoomListRequest{
}