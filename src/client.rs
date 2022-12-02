use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

use crate::common::{self, send_message};
use crate::network_message::{MessageType, NetworkError, NetworkMessage};
use crate::request;
use crate::room::Room;

static mut USER_ID: i32 = 0;

pub async fn run(ip: &str, port: &str) {
    let mut user_input_buf: String = String::new();
    let stdin = std::io::stdin();

    let mut skip_waiting_for_server = false;

    let mut read_buf: Vec<u8> = vec![0 as u8; 1024];

    let mut stream: TcpStream = TcpStream::connect(format!("{}:{}", ip, port))
        .await
        .unwrap();

    loop {
        if !skip_waiting_for_server {
            let new_message: NetworkMessage =
                wait_for_server_message(&mut stream, &mut read_buf).await;
            println!("new message: {:?}", new_message);
            handle_incoming_message(&mut stream, &new_message).await;
        } else {
            skip_waiting_for_server = false;
        }

        println!("<------------------------------->");
        println!(
            "what would you like to do? {{ 1 - show rooms, 2 - create a new room , 3 - join room}}"
        );
        stdin.read_line(&mut user_input_buf).unwrap();

        if user_input_buf.trim_end() == "1" {
            println!("fetching rooms...");
            get_rooms(&mut stream).await;
        } else if user_input_buf.trim_end() == "2" {
            println!("creating new room...");
            send_message(
                NetworkMessage::new("".to_string(), MessageType::CreateRoomRequest),
                &mut stream,
            )
            .await;
        } else if user_input_buf.trim_end() == "3" {
            match join_room(&mut stream).await {
                Ok(_) => {}
                Err(_) => {
                    skip_waiting_for_server = true;
                }
            };
        }
        user_input_buf = "".to_string();
    }
}

async fn handle_incoming_message(_stream: &mut TcpStream, message: &NetworkMessage) {
    match message.message_type {
        MessageType::UserJoinedResponse(data) => unsafe { USER_ID = data.user_id },
        MessageType::RoomListResponse => {
            let response = message.clone();
            let json = &response.text;
            let room_list = serde_json::from_str::<Vec<Room>>(json.as_str()).unwrap();

            println!(
                "received room IDs: {:#?}",
                room_list.iter().map(|r| r.room_id).collect::<Vec<i32>>()
            );
        }
        MessageType::CreateRoomResponse => {
            println!("successfully created a new room");
        }
        MessageType::JoinRoomResponse => todo!(),
        MessageType::Error(error) => match error {
            NetworkError::RoomWithIdAlreadyExists => println!("you already created a room"),
        },
        MessageType::Other => {
            println!("other message: {:#?}", message.text);
        }
        _ => (),
    };
}

async fn get_rooms(stream: &mut TcpStream) {
    let message = NetworkMessage::new(
        serde_json::to_string(&request::RoomListRequest {})
            .unwrap()
            .trim()
            .to_string(),
        MessageType::RoomListRequest,
    );
    send_message(message, stream).await;
}

async fn join_room(_stream: &mut TcpStream) -> Result<(), ()> {
    println!("input room id");
    let tmp_stdin = std::io::stdin();
    let mut tmp_buf = String::new();
    tmp_stdin.read_line(&mut tmp_buf).unwrap();
    //TODO:
    // if tmp_buf in ROOM_LIST
    println!("joined room {}", tmp_buf);
    return Err(());
}

async fn wait_for_server_message(stream: &mut TcpStream, buffer: &mut Vec<u8>) -> NetworkMessage {
    common::clear_buffer(buffer);
    stream
        .read(buffer)
        .await
        .expect("failed to read data from socket");
    let json = std::str::from_utf8(buffer).unwrap().replace('\0', "");
    let message = serde_json::from_str::<NetworkMessage>(json.as_str())
        .expect("error while trying to parse incoming data");
    return message;
}
