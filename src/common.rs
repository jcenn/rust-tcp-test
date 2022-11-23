use serde_derive::{Deserialize, Serialize};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

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

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    Other,
}

pub async fn send_message(message: &NetworkMessage, stream: &mut TcpStream) -> Result<(), ()> {
    let json: String = serde_json::to_string(message).unwrap();
    match stream.write(json.as_bytes()).await {
        Ok(_) => {
            return Ok(());
        }
        Err(_) => {
            return Err(());
        }
    }
}

pub async fn wait_for_message(stream: &mut TcpStream, buffer: &mut [u8]) -> NetworkMessage {
    let n = stream
        .read(buffer)
        .await
        .expect("failed to read data from socket");
    let json = std::str::from_utf8(buffer).unwrap().replace('\0', "");
    let message = serde_json::from_str::<NetworkMessage>(json.as_str())
        .expect("error while trying to parse incoming data");
    println!("received {:?}", message.text);
    return message;
}
