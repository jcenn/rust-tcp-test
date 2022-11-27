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
    JoinRoomRequest,
    JoinRoomResponse,
    RoomListRequest,
    RoomListResponse,
    Other,
}
