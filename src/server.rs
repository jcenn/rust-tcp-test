use std::io::ErrorKind;

use tokio::net::{TcpListener, TcpStream};

use crate::common::{send_message, wait_for_client_message, Connection, Room};
use crate::network_message::{MessageType, NetworkMessage};
use crate::request;

static mut room_list: Vec<Room> = vec![];

pub async fn run(ip: &str, port: &str) {
    unsafe {
        room_list.push(Room {
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
        .await
        .unwrap();

        //new thread for each connection
        tokio::spawn(async move {
            let current_connection = new_connection;
            let mut received_buffer = vec![0; 1024];
            let mut last_message: NetworkMessage =
                NetworkMessage::new("".to_string(), MessageType::Other);

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
                            panic!("zoinks scoob something is broken");
                        }
                    },
                };

                if new_received.message_type != last_message.message_type
                    || new_received.text != last_message.text
                {
                    handle_new_message(&new_received, &current_connection, &mut stream)
                        .await
                        .unwrap();

                    last_message = new_received;
                }
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
            let request: request::RoomListRequest =
                serde_json::from_str::<request::RoomListRequest>(message.text.as_str().trim())
                    .unwrap();
            unsafe {
                send_message(
                    &NetworkMessage::new(
                        serde_json::to_string(&room_list).unwrap(),
                        MessageType::RoomListResponse,
                    ),
                    stream,
                )
                .await
                .unwrap();
            }
        }
        MessageType::JoinRoomRequest => {
            let request =
                serde_json::from_str::<request::JoinRoomRequest>(message.text.as_str().trim())
                    .unwrap();
            if room_with_id_exists(request.room_id) {
                unsafe {
                    let index = get_room_index(request.room_id);
                    let room = &mut room_list[index as usize];
                    room.connections.push(connection.to_owned());
                }
            } else {
                println!("oh no")
            }
        }
        MessageType::Other => {
            if message.text == "hello there".to_string() {
                send_message(
                    &NetworkMessage::new("general kenobi".to_string(), MessageType::Other),
                    stream,
                )
                .await
                .unwrap();
            }
        }
        MessageType::JoinRoomResponse => (),
        MessageType::RoomListResponse => (),
    }

    return Ok(());
}

fn room_with_id_exists(id: i32) -> bool {
    unsafe {
        let filtered = room_list
            .iter()
            .filter(|r| r.room_id == id)
            .collect::<Vec<&Room>>();
        return filtered.len() > 0;
    }
}

fn get_room_index(id: i32) -> i32 {
    unsafe {
        for i in 0..room_list.len() {
            if room_list[i].room_id == id {
                return i as i32;
            }
        }
        return -1;
    }
}

async fn create_new_room(host: &Connection) -> Room {
    return Room {
        connections: vec![host.to_owned()],
        //TODO: generate separate room id
        room_id: host.id,
    };
}
