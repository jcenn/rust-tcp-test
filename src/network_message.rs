use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum MessageType {
    UserJoinedResponse(UserJoinedResponseDto),
    JoinRoomRequest,
    JoinRoomResponse,
    RoomListRequest,
    RoomListResponse,
    CreateRoomRequest,
    CreateRoomResponse,
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
}
