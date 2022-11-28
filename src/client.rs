use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

use crate::common::send_message;
use crate::network_message::{MessageType, NetworkMessage};
use crate::request;
use crate::room::Room;

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

    // let message = NetworkMessage::new(
    //     serde_json::to_string(&JoinRoomRequest { room_id: 42 }).unwrap(),
    //     MessageType::JoinRoomRequest,
    // );
    // send_message(&message, &mut stream).await;

    loop {
        let new_message: NetworkMessage = wait_for_server_message(&mut stream, &mut read_buf).await;
        println!("new message: {:?}", new_message);
        // if new_message.message_type != last_message.message_type
        //     || new_message.text != last_message.text
        // {
        handle_incoming_message(&mut stream, &new_message).await;

        println!("what would you like to do? {{ 1 - show rooms, 2 - create a new room }}");
        stdin.read_line(&mut user_input_buf).unwrap();

        if user_input_buf.trim_end() == "1" {
            println!("fetching rooms...");
            get_rooms(&mut stream, &mut read_buf).await;
        } else if user_input_buf.trim_end() == "2" {
            println!("creating new room...");
            send_message(
                &NetworkMessage::new("".to_string(), MessageType::CreateRoomRequest),
                &mut stream,
            )
            .await;
        } else if user_input_buf.trim_end() == "3" {
            //refresh
            continue;
        }
        user_input_buf = "".to_string();
        last_message = new_message;
        // }
    }
}

async fn get_rooms(stream: &mut TcpStream, read_buf: &mut Vec<u8>) {
    let message = NetworkMessage::new(
        serde_json::to_string(&request::RoomListRequest {})
            .unwrap()
            .trim()
            .to_string(),
        MessageType::RoomListRequest,
    );
    send_message(&message, stream).await;
}

//TODO:
async fn handle_incoming_message(stream: &mut TcpStream, message: &NetworkMessage) {
    match message.message_type {
        MessageType::JoinRoomResponse => todo!(),
        MessageType::RoomListResponse => {
            let response = message.clone();
            let json = &response.text;
            let room_list = serde_json::from_str::<Vec<Room>>(json.as_str()).unwrap();

            println!("received rooms: {:#?}", room_list);
        }
        MessageType::CreateRoomResponse => {
            println!("successfully created a new room");
        }
        MessageType::Other => {
            println!("message: {:#?}", message.text);
        }
        MessageType::JoinRoomRequest => todo!(),
        MessageType::RoomListRequest => todo!(),
        MessageType::CreateRoomRequest => todo!(),
        MessageType::Error => todo!(),
    }
}

//TODO:
// async fn connect_to_room(
//     stream: &mut TcpStream,
//     read_buf: &mut Vec<u8>,
//     id: i32,
// ) -> Result<(), ()> {
//     return Ok(());
// }

pub async fn wait_for_server_message(
    stream: &mut TcpStream,
    buffer: &mut Vec<u8>,
) -> NetworkMessage {
    stream
        .read(buffer)
        .await
        .expect("failed to read data from socket");
    let json = std::str::from_utf8(buffer).unwrap().replace('\0', "");
    let message = serde_json::from_str::<NetworkMessage>(json.as_str())
        .expect("error while trying to parse incoming data");
    return message;
}
