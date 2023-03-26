use std::{io::Error, io::ErrorKind};

use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};

use crate::network_message::{MessageType, NetworkMessage, UserJoinedResponseDto};
use crate::room::Room;
use crate::{
    common::{self, send_message},
    room::RoomUser,
};
use crate::{connection::Connection, network_message::NetworkError};

#[derive(PartialEq, Eq, Clone)]
struct QueueMessage {
    receiver_id: i32,
    message: NetworkMessage,
}

static mut ROOM_LIST: Vec<Room> = vec![];
static mut MESSAGE_QUEUE: Vec<QueueMessage> = vec![];

pub async fn run(ip: &str, port: &str) {
    //for testing
    // unsafe {
    //     ROOM_LIST.push(Room {
    //         connections: vec![],
    //         room_id: 42,
    //         players: vec![]
    //     });
    // }

    let listener = TcpListener::bind(format!("{}:{}", ip, port)).await.unwrap();
    println!("running as server on {}:{}", ip, port);

    loop {
        let (mut stream, addr) = listener.accept().await.unwrap();
        let new_connection: Connection = Connection::new(addr.ip(), addr.port());
        println!("new connection from {}", new_connection.ip);
        send_message(
            NetworkMessage::new(
                format!("your id: {}", new_connection.id),
                MessageType::UserConnectedResponse(UserJoinedResponseDto {
                    user_id: new_connection.id,
                }),
            ),
            &mut stream,
        )
        .await;

        //new thread for each connection
        tokio::spawn(async move {
            let current_connection = new_connection;
            let mut received_buffer = vec![0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                unsafe {
                    let messages = MESSAGE_QUEUE
                        .iter()
                        .filter(|m| m.receiver_id == current_connection.id)
                        .collect::<Vec<&QueueMessage>>();
                    //TODO: es ist nicht working
                    // if messages.len() > 0 {
                    //     send_message(messages[0].message.clone(), &mut stream).await;
                    //     let mut found_message_index: i32 = -1;
                    //     for i in 0..MESSAGE_QUEUE.len() {
                    //         if MESSAGE_QUEUE[i] == messages[0].clone() {
                    //             found_message_index = i as i32;
                    //             break;
                    //         }
                    //     }
                    //     if found_message_index == -1 {
                    //         panic!("o no")
                    //     }
                    //     MESSAGE_QUEUE.remove(found_message_index as usize);
                    // }
                }
                let new_received = match wait_for_client_message(
                    &mut stream,
                    &mut received_buffer,
                    &current_connection,
                )
                .await
                {
                    Ok(message) => message,
                    Err(err) => match err {
                        ServerError::ClientDisconnected => {
                            println!(
                                "connection aborted on {}:{}",
                                current_connection.ip, current_connection.port
                            );
                            break;
                        }
                        ServerError::Other(err) => {
                            println!("{:?}", err);
                            // panic!("zoinks scoob something is broken");
                            panic!();
                        }
                        _ => {
                            panic!("unknown error")
                        }
                    },
                };
                println!("new message: {:?}", new_received);

                handle_new_message(&new_received, &current_connection, &mut stream)
                    .await
                    .unwrap();
            }
        });
    }
}

async fn handle_new_message(
    message: &NetworkMessage,
    connection: &Connection,
    stream: &mut TcpStream,
) -> Result<(), ()> {
    match &message.message_type {
        MessageType::RoomListRequest => {
            println!(
                "client with id:{} ({}:{}) requested room list",
                connection.id, connection.ip, connection.port
            );
            unsafe {
                send_message(
                    NetworkMessage::new(
                        serde_json::to_string(&ROOM_LIST).unwrap(),
                        MessageType::RoomListResponse,
                    ),
                    stream,
                )
                .await;
            }
        }
        MessageType::CreateRoomRequest => {
            match create_new_room(&connection).await {
                Ok(_) => {
                    send_message(
                        NetworkMessage::new("".to_string(), MessageType::CreateRoomResponse),
                        stream,
                    )
                    .await;
                    println!("user with id:{} created a new room", connection.id);
                }
                Err(_) => {
                    send_message(
                        NetworkMessage::new(
                            "".to_string(),
                            MessageType::Error(NetworkError::RoomWithIdAlreadyExists),
                        ),
                        stream,
                    )
                    .await;
                }
            };
        }
        MessageType::JoinRoomRequest(selected_room_id) => {
            println!(
                "user with id:{} tries to join room with id:{}",
                connection.id, selected_room_id
            );
            if !room_with_id_exists(selected_room_id.clone()) {
                send_message(
                    NetworkMessage::new(
                        String::new(),
                        MessageType::Error(NetworkError::CouldNotConnectToGivenRoom),
                    ),
                    stream,
                )
                .await;

                panic!(
                    "client with id:{} tried to connect to non-existing room (id:{:?})",
                    connection.id, selected_room_id
                )
            }

            if user_is_in_room(connection.id, selected_room_id.clone()) {
                send_message(
                    NetworkMessage::new(
                        String::new(),
                        MessageType::Error(NetworkError::UserIsAlreadyConnectedToRoom),
                    ),
                    stream,
                )
                .await;

                panic!(
                    "client with id:{} tried to connect to a room they're already connected to (id:{:?})",
                    connection.id, selected_room_id
                )
            }

            unsafe {
                let index = get_room_index(selected_room_id.clone());
                let room = &mut ROOM_LIST[index as usize];
                room.connections.push(connection.to_owned());
            }
            send_message(
                NetworkMessage::new(String::new(), MessageType::JoinRoomResponse),
                stream,
            )
            .await;
        }
        MessageType::SelectMove(move_type) => unsafe {
            let found_room = ROOM_LIST
                .iter()
                .filter(|r| r.connections.contains(connection))
                .collect::<Vec<&Room>>()[0];
            let opponent_id = found_room
                .players
                .iter()
                .filter(|p| p.player_id != connection.id)
                .collect::<Vec<&RoomUser>>()[0]
                .player_id;

            MESSAGE_QUEUE.push(QueueMessage {
                receiver_id: opponent_id,
                message: NetworkMessage {
                    text: "".to_string(),
                    message_type: MessageType::OpponentMove(move_type.clone()),
                },
            })
        },
        MessageType::Other => {
            if message.text == "hello there".to_string() {
                send_message(
                    NetworkMessage::new("general kenobi".to_string(), MessageType::Other),
                    stream,
                )
                .await;
            }
        }
        _ => (),
    }

    return Ok(());
}

fn user_is_in_room(user_id: i32, room_id: i32) -> bool {
    let index = get_room_index(room_id);
    if index == -1 {
        return false;
    }

    let room: &Room;
    unsafe {
        room = &ROOM_LIST[index as usize];
    }
    if room.connections.iter().filter(|c| c.id == user_id).count() > 0 {
        return true;
    }
    return false;
}

fn room_with_id_exists(id: i32) -> bool {
    unsafe {
        let filtered = ROOM_LIST
            .iter()
            .filter(|r| r.room_id == id)
            .collect::<Vec<&Room>>();
        return filtered.len() > 0;
    }
}

#[doc = "returns found index or -1 if nothing has been found"]
fn get_room_index(id: i32) -> i32 {
    unsafe {
        for i in 0..ROOM_LIST.len() {
            if ROOM_LIST[i].room_id == id {
                return i as i32;
            }
        }
        return -1;
    }
}

async fn create_new_room(host: &Connection) -> Result<(), ServerError> {
    let room_id = host.id;
    if room_with_id_exists(room_id) {
        println!("room with id {room_id} already exists");
        return Err(ServerError::UserAlreadyCreatedARoom);
    }
    unsafe {
        ROOM_LIST.push(Room {
            connections: vec![host.to_owned()],
            room_id: host.id,
            players: vec![RoomUser {
                player_id: host.id,
                selected_move: None,
            }],
        });
    };
    return Ok(());
}

pub async fn wait_for_client_message(
    stream: &mut TcpStream,
    buffer: &mut Vec<u8>,
    _connection: &Connection,
) -> Result<NetworkMessage, ServerError> {
    common::clear_buffer(buffer);
    match stream.read(buffer).await {
        Ok(_) => {}
        Err(err) => match err.kind() {
            ErrorKind::ConnectionReset | ErrorKind::ConnectionAborted => {
                return Err(ServerError::ClientDisconnected);
            }
            _ => {
                return Err(ServerError::Other(err));
            }
        },
    };
    let json = std::str::from_utf8(buffer).unwrap().replace('\0', "");
    let message = serde_json::from_str::<NetworkMessage>(json.as_str())
        .expect("error while trying to parse incoming data");
    return Ok(message);
}

pub enum ServerError {
    UserAlreadyCreatedARoom,
    ClientDisconnected,
    Other(Error),
}
