use serde_derive::{Deserialize, Serialize};

use crate::connection::Connection;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub room_id: i32,
    pub connections: Vec<Connection>,
}
