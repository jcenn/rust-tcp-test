use serde_derive::{Deserialize, Serialize};
use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::network_message::NetworkMessage;

pub async fn send_message(message: NetworkMessage, stream: &mut TcpStream) {
    let json: String = serde_json::to_string(&message).unwrap();
    match stream.write(json.as_bytes()).await {
        Ok(_) => {}
        Err(_) => {
            panic!("couldn't send message: {:#?}", message);
        }
    }
}

pub fn clear_buffer(buffer: &mut Vec<u8>) {
    for i in 0..buffer.len() {
        buffer[i] = 0 as u8;
    }
}
#[derive(Debug, Serialize,Deserialize, PartialEq, Eq, Clone)]
pub enum MoveType {
    Rock,
    Paper,
    Scissors,
}

pub enum ClientState{
    NotConnected,
    Connected,
    InLobby,
    Playing
}