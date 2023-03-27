use serde_derive::{Deserialize, Serialize};

use crate::{common::MoveType, room::Room};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct NetworkMessage {
    pub text: String,
    pub message_type: MessageType,
}

impl NetworkMessage {
    pub fn new(message: String, message_type: MessageType) -> Self {
        Self {
            text: message,
            message_type: message_type,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum MessageType {
    UserConnectedResponse(UserJoinedResponseDto),
    JoinRoomRequest(i32),
    JoinRoomResponse(Room),
    RoomListRequest,
    RoomListResponse,
    CreateRoomRequest,
    CreateRoomResponse,
    ClientListRequest,
    ClientListResponse(Vec<i32>),
    RoomUpdate(Room),
    SelectMove(MoveType),
    OpponentMove(MoveType),
    Error(NetworkError),
    Other,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub struct UserJoinedResponseDto {
    pub user_id: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum NetworkError {
    RoomWithIdAlreadyExists,
    UserIsAlreadyConnectedToRoom,
    CouldNotConnectToGivenRoom,
    RoomIsFull
}
