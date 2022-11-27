use std::io::{BufReader, Read};

use tokio::{io, net::TcpStream};

use crate::common::{send_message, wait_for_server_message, Room};
use crate::network_message::{MessageType, NetworkMessage};
use crate::request::{self, JoinRoomRequest};

pub async fn run(ip: &str, port: &str) {
    let mut user_input_buf: String = String::new();
    let stdin = std::io::stdin();
    // let stdin = io::stdin();

    // let mut input_buf: String = String::new();
    let mut read_buf: Vec<u8> = vec![0 as u8; 1024];

    let mut stream: TcpStream = TcpStream::connect(format!("{}:{}", ip, port))
        .await
        .unwrap();

    let mut last_message: NetworkMessage = NetworkMessage::new("".to_string(), MessageType::Other);
    // send_message(
    //     &NetworkMessage::new("hello there".to_string(), MessageType::Other),
    //     &mut stream,
    // )
    // .await
    // .unwrap();

    // send_message(
    //     &NetworkMessage::new(String::from("wth man"), MessageType::RoomListRequest),
    //     &mut stream,
    // )
    // .await
    // .unwrap();

    let message = NetworkMessage::new(
        serde_json::to_string(&JoinRoomRequest { room_id: 42 }).unwrap(),
        MessageType::JoinRoomRequest,
    );
    send_message(&message, &mut stream).await.unwrap();

    loop {
        let new_message: NetworkMessage = wait_for_server_message(&mut stream, &mut read_buf).await;
        println!("new message: {:?}", new_message);
        if new_message.text != last_message.text {
            println!("what would you like to do? {{ 1 - show rooms, 2 - create a new room }}");
            stdin.read_line(&mut user_input_buf).unwrap();

            if user_input_buf.trim_end() == "1" {
                println!("fetching rooms...");
                let rooms = get_rooms(&mut stream, &mut read_buf).await;
                println!("rooms: {:?}", rooms);
            }
            user_input_buf = "".to_string();
            last_message = new_message;
        }
    }
}

async fn get_rooms(stream: &mut TcpStream, read_buf: &mut Vec<u8>) -> Vec<Room> {
    let message = NetworkMessage::new(
        serde_json::to_string(&request::RoomListRequest {})
            .unwrap()
            .trim()
            .to_string(),
        MessageType::RoomListRequest,
    );
    send_message(&message, stream).await.unwrap();
    let response = wait_for_server_message(stream, read_buf).await;
    let json = response.text;
    let data = serde_json::from_str::<Vec<Room>>(json.as_str()).unwrap();

    return data;
}

async fn handle_incoming_message(stream: &mut TcpStream, message: &NetworkMessage) {
    match message.message_type {
        MessageType::JoinRoomResponse => todo!(),
        MessageType::RoomListResponse => todo!(),
        MessageType::Other => todo!(),
        MessageType::JoinRoomRequest => todo!(),
        MessageType::RoomListRequest => todo!(),
    }
}

async fn connect_to_room(
    stream: &mut TcpStream,
    read_buf: &mut Vec<u8>,
    id: i32,
) -> Result<(), ()> {
    return Ok(());
}
