use std::io::Stdin;

use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

use crate::common::{self, send_message, ClientState, MoveType};
use crate::network_message::{MessageType, NetworkError, NetworkMessage};
use crate::request;
use crate::room::Room;

static mut USER_ID: i32 = 0;
static mut ROOM_LIST: Vec<Room> = vec![];
// static mut IS_CURRENTLY_IN_ROOM: bool = false;
static mut SKIP_WAITING_FOR_SERVER: bool = false;
static mut CURRENT_STATE: ClientState = ClientState::NotConnected;

pub async fn run(ip: &str, port: &str) {
    let mut user_input_buf: String = String::new();
    let stdin = std::io::stdin();

    unsafe {
        SKIP_WAITING_FOR_SERVER = false;
    }
    let mut read_buf: Vec<u8> = vec![0 as u8; 1024];

    println!("press enter to connect to the server...");
    let _ = stdin.read_line(&mut user_input_buf).unwrap();
    user_input_buf.clear();

    let mut stream: TcpStream = TcpStream::connect(format!("{}:{}", ip, port))
        .await
        .unwrap();
    unsafe { CURRENT_STATE = ClientState::Connected };

    loop {
        //clears terminal
        //print!("{}[2J", 27 as char);

        // let should_skip: bool;
        // unsafe {
        //     should_skip = SKIP_WAITING_FOR_SERVER;
        // }
        // if should_skip {
        //     unsafe {
        //         SKIP_WAITING_FOR_SERVER = false;
        //     }
        // } else {
        let new_message: NetworkMessage = wait_for_server_message(&mut stream, &mut read_buf).await;
        println!("new message: {:?}", new_message);
        handle_incoming_message(&mut stream, &new_message).await;
        // }

        unsafe {
            match CURRENT_STATE {
                ClientState::Connected => {
                    ask_user_for_input(&mut stream, &stdin, &mut user_input_buf).await;
                }
                ClientState::Playing => {
                    ask_user_for_game_input(&mut stream, &stdin, &mut user_input_buf).await;
                }
                _ => (),
            }
        }

        user_input_buf.clear();
    }
}

async fn handle_incoming_message(_stream: &mut TcpStream, message: &NetworkMessage) {
    match &message.message_type {
        MessageType::UserConnectedResponse(data) => unsafe { USER_ID = data.user_id },
        MessageType::RoomListResponse => {
            let response = message.clone();
            let json = &response.text;
            let tmp_room_list = serde_json::from_str::<Vec<Room>>(json.as_str()).unwrap();
            unsafe {
                ROOM_LIST = tmp_room_list.clone();
            }
            println!("received rooms: {:#?}", tmp_room_list);
        }
        MessageType::CreateRoomResponse => {
            println!("successfully created a new room");
            unsafe{
                CURRENT_STATE = ClientState::InLobby;
                println!("waiting for another user to join...")
            }
        }
        MessageType::JoinRoomResponse => {
            println!("successfully joined selected room");
            unsafe {
                CURRENT_STATE = ClientState::InLobby;
                println!("waiting for another user to join...")

            }
        }
        MessageType::OpponentMove(move_type) => {
            println!("opponent used {:?}", move_type);
        }
        #[allow(unreachable_patterns)]
        MessageType::Error(error) => match error {
            NetworkError::RoomWithIdAlreadyExists => println!("you already created a room"),
            NetworkError::CouldNotConnectToGivenRoom => println!("could not connect to this room"),
            NetworkError::UserIsAlreadyConnectedToRoom => {
                todo!("you're already connected to this room")
            }
            _ => panic!("unknown error"),
        },
        MessageType::Other => {
            println!("other message: {:#?}", message.text);
        }
        _ => (),
    };
}

#[doc = "tells server to return list of current rooms"]
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

async fn join_room(stream: &mut TcpStream) -> Result<(), String> {
    let mut tmp_buffer = vec![0 as u8; 1024];

    println!("input room id");
    let tmp_stdin = std::io::stdin();
    let mut tmp_buf = String::new();
    tmp_stdin.read_line(&mut tmp_buf).unwrap();
    let selected_id: i32;
    match tmp_buf.trim().parse::<i32>() {
        Ok(parsed_id) => {
            selected_id = parsed_id;
        }
        Err(_) => {
            return Err(format!(
                "couldn't parse given room id as i32 (id:{})",
                tmp_buf
            ));
        }
    }
    let room_count;
    unsafe {
        room_count = ROOM_LIST.len();
    }
    if room_count <= 0 {
        get_rooms(stream).await;
        let received_message = wait_for_server_message(stream, &mut tmp_buffer).await;
        unsafe {
            ROOM_LIST = serde_json::from_str::<Vec<Room>>(&received_message.text.as_str()).unwrap();
            SKIP_WAITING_FOR_SERVER = true;
        }
    }

    unsafe {
        let room_exists = ROOM_LIST
            .iter()
            .filter(|r| r.room_id == selected_id)
            .collect::<Vec<&Room>>()
            .len()
            > 0;

        if room_exists {
            send_message(
                NetworkMessage::new("".to_string(), MessageType::JoinRoomRequest(selected_id)),
                stream,
            )
            .await;
            return Ok(());
        }

        return Err(String::from("room with given id doesn't exist"));
    }
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

async fn ask_user_for_input(stream: &mut TcpStream, stdin: &Stdin, user_input_buf: &mut String) {
    println!("<------------------------------->");
    println!(
        "what would you like to do?: \n1 - show rooms \n2 - create a new room \n3 - join a room"
    );
    stdin.read_line(user_input_buf).unwrap();

    let chosen_option = user_input_buf.trim().parse::<i32>().unwrap();
    match chosen_option {
        1 => {
            println!("fetching rooms...");
            get_rooms(stream).await;
        }
        2 => {
            println!("creating new room...");
            send_message(
                NetworkMessage::new("".to_string(), MessageType::CreateRoomRequest),
                stream,
            )
            .await;
        }
        3 => {
            match join_room(stream).await {
                Ok(_) => {}
                Err(err) => {
                    println!("error: {:#?}", err);
                    //TODO: test if it works
                    unsafe {
                        SKIP_WAITING_FOR_SERVER = true;
                    }
                }
            };
        }
        _ => {
            println!("unknown command");
            unsafe {
                SKIP_WAITING_FOR_SERVER = true;
            }
        }
    }
}

async fn ask_user_for_game_input(
    stream: &mut TcpStream,
    stdin: &Stdin,
    user_input_buf: &mut String,
) {
    println!("<------------------------------->");
    println!("what would you like to do: \n 1)rock \n 2)paper \n 3)scissors");

    stdin.read_line(user_input_buf).unwrap();

    let chosen_option = user_input_buf.trim().parse::<i32>().unwrap();
    match chosen_option {
        1 => {
            println!("selected rock");
            send_message(
                NetworkMessage::new("".to_string(), MessageType::SelectMove(MoveType::Rock)),
                stream,
            )
            .await;
        }
        2 => {
            println!("selected paper");
            send_message(
                NetworkMessage::new("".to_string(), MessageType::SelectMove(MoveType::Paper)),
                stream,
            )
            .await;
        }
        3 => {
            println!("selected scissors");
            send_message(
                NetworkMessage::new("".to_string(), MessageType::SelectMove(MoveType::Scissors)),
                stream,
            )
            .await;
        }
        _ => {
            println!("unknown command");
            unsafe {
                SKIP_WAITING_FOR_SERVER = true;
            }
        }
    }
}
