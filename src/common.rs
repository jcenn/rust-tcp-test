use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkMessage {
    pub text: String,
}

impl NetworkMessage {
    pub fn new(message: String) -> Self {
        Self { text: message }
    }
}

//TODO: copy from server.rs / client.rs
pub fn send_message(){

}

//TODO: copy from server.rs / client.rs
pub fn wait_for_message(){

}