use std::{io::Error, io::ErrorKind};

use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};

use crate::connection::Connection;
use crate::network_message::{MessageType, NetworkMessage};
use crate::request;
use crate::room::Room;
use crate::{common::send_message, connection};

static mut ROOM_LIST: Vec<Room> = vec![];

pub async fn run(ip: &str, port: &str) {
    unsafe {
        ROOM_LIST.push(Room {
            connections: vec![],
            room_id: 42,
        });
    }
    let listener = TcpListener::bind(format!("{}:{}", ip, port)).await.unwrap();
    println!("running as server on {}:{}", ip, port);

    loop {
        let (mut stream, addr) = listener.accept().await.unwrap();
        let new_connection: Connection = Connection::new(addr.ip(), addr.port());
        println!("new connection from {}", new_connection.ip);
        send_message(
            &NetworkMessage::new(
                format!("your id: {}", new_connection.id),
                MessageType::Other,
            ),
            &mut stream,
        )
        .await;

        //new thread for each connection
        tokio::spawn(async move {
            let current_connection = new_connection;
            let mut received_buffer = vec![0; 1024];
            // let mut last_message: NetworkMessage =
            //     NetworkMessage::new("".to_string(), MessageType::Other);

            // In a loop, read data from the socket and write the data back.
            loop {
                let new_received = match wait_for_client_message(
                    &mut stream,
                    &mut received_buffer,
                    &current_connection,
                )
                .await
                {
                    Ok(message) => message,
                    Err(err) => match err.kind() {
                        ErrorKind::ConnectionAborted | ErrorKind::ConnectionReset => {
                            println!(
                                "connection aborted on {}:{}",
                                current_connection.ip, current_connection.port
                            );
                            break;
                        }
                        _ => {
                            println!("{:?}", err);
                            // panic!("zoinks scoob something is broken");
                            panic!();
                        }
                    },
                };
                println!("new message: {:?}", new_received);
                // if new_received.message_type != last_message.message_type
                //     || new_received.text != last_message.text
                // {
                handle_new_message(&new_received, &current_connection, &mut stream)
                    .await
                    .unwrap();

                // last_message = new_received;
                // }
            }
        });
    }
}

async fn handle_new_message(
    message: &NetworkMessage,
    connection: &Connection,
    stream: &mut TcpStream,
) -> Result<(), ()> {
    match message.message_type {
        MessageType::RoomListRequest => {
            // let request: request::RoomListRequest =
            //     serde_json::from_str::<request::RoomListRequest>(message.text.as_str().trim())
            //         .unwrap();
            println!(
                "client with id:{} ({}:{}) requested room list",
                connection.id, connection.ip, connection.port
            );
            unsafe {
                send_message(
                    &NetworkMessage::new(
                        serde_json::to_string(&ROOM_LIST).unwrap(),
                        MessageType::RoomListResponse,
                    ),
                    stream,
                )
                .await;
            }
        }
        MessageType::CreateRoomRequest => {
            create_new_room(&connection, stream).await;
            println!("user with id:{} created a new room", connection.id);
        }
        MessageType::JoinRoomRequest => {
            let request: request::JoinRoomRequest =
                serde_json::from_str::<request::JoinRoomRequest>(message.text.as_str().trim())
                    .unwrap();
            println!(
                "user with id:{} tries to join room with id:{}",
                connection.id, request.room_id
            );
            if room_with_id_exists(request.room_id) {
                unsafe {
                    let index = get_room_index(request.room_id);
                    let room = &mut ROOM_LIST[index as usize];
                    room.connections.push(connection.to_owned());
                }
            } else {
                panic!(
                    "client with id:{} tried to connect to non-existing room (id:{:?})",
                    connection.id, request.room_id
                )
            }
        }

        MessageType::Other => {
            if message.text == "hello there".to_string() {
                send_message(
                    &NetworkMessage::new("general kenobi".to_string(), MessageType::Other),
                    stream,
                )
                .await;
            }
        }
        _ => (),
    }

    return Ok(());
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

async fn create_new_room(host: &Connection, stream: &mut TcpStream) {
    let room_id = host.id;
    if room_with_id_exists(room_id) {
        //TODO:
        todo!("send error message to client");
    }
    unsafe {
        ROOM_LIST.push(Room {
            connections: vec![host.to_owned()],
            room_id: host.id,
        });
    };
    let response = NetworkMessage::new("".to_string(), MessageType::CreateRoomResponse);
    send_message(&response, stream).await;
}

pub async fn wait_for_client_message(
    stream: &mut TcpStream,
    buffer: &mut Vec<u8>,
    connection: &Connection,
) -> Result<NetworkMessage, Error> {
    //reset buffer
    // for i in 0..buffer.len() {
    //     buffer[i] = 0 as u8;
    // }
    // match stream.read(buffer).await {
    //     Ok(size) => size,
    //     Err(err) => {
    //         println!("error: {:?}", err);
    //         match err.kind() {
    //             ErrorKind::ConnectionReset | ErrorKind::ConnectionAborted => {
    //                 println!(
    //                     "connection with client_id:{} ({}:{}) has been aborted",
    //                     connection.id, connection.ip, connection.port
    //                 );
    //                 return Err(err);
    //             }
    //             other => {
    //                 println!("dunno what happened bro");
    //                 println!("{}", other);
    //                 panic!();
    //             }
    //         }
    //     }
    // };
    // println!("buffer: {:#?}", buffer);
    // let json = std::str::from_utf8(buffer).unwrap().replace('\0', "");
    // let error_message = format!(
    //     "error while trying to parse incoming data, json: {:#?}",
    //     json.to_string()
    // );
    // let message = serde_json::from_str::<NetworkMessage>(json.as_str()).expect(&error_message);
    // return Ok(message);
    stream
        .read(buffer)
        .await
        .expect("failed to read data from socket");
    let json = std::str::from_utf8(buffer).unwrap().replace('\0', "");
    let message = serde_json::from_str::<NetworkMessage>(json.as_str())
        .expect("error while trying to parse incoming data");
    return Ok(message);
}
