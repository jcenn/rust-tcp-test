use std::{io::Error, io::ErrorKind, net::IpAddr};

use serde_derive::{Deserialize, Serialize};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

static mut last_id: i32 = 0;

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
    JoinRoom,
    SendRoomList,
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

pub async fn wait_for_server_message(stream: &mut TcpStream, buffer: &mut [u8]) -> NetworkMessage {
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

pub async fn wait_for_client_message(
    stream: &mut TcpStream,
    buffer: &mut [u8],
    connection: &Connection,
) -> Result<NetworkMessage, Error> {
    let n = match stream.read(buffer).await {
        Ok(size) => size,
        Err(err) => {
            println!("error: {:?}", err);
            match err.kind() {
                ErrorKind::ConnectionReset | ErrorKind::ConnectionAborted => {
                    println!("connection has been aborted");
                    return Err(err);
                }
                other => {
                    println!("dunno what happened bro");
                    println!("{}", other);
                    panic!();
                }
            }
        }
    };
    let json = std::str::from_utf8(buffer).unwrap().replace('\0', "");
    let message = serde_json::from_str::<NetworkMessage>(json.as_str())
        .expect("error while trying to parse incoming data");
    println!("received {:?} from id: {}", message.text, connection.id);
    return Ok(message);
}

pub struct Room {
    pub room_id: i32,
    pub connections: Vec<Connection>,
}

pub struct Connection {
    pub ip: IpAddr,
    pub port: u16,
    pub id: i32,
}

impl Connection {
    pub fn new(ip: IpAddr, port: u16) -> Self {
        unsafe {
            last_id += 1;
            Self {
                ip: ip,
                port: port,
                id: last_id,
            }
        }
    }
}
