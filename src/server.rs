use std::io::ErrorKind;

use serde::de::value::Error;
use tokio::net::{TcpListener, TcpStream};

use crate::common::{
    send_message, wait_for_client_message, Connection, MessageType, NetworkMessage,
};

pub async fn run(ip: &str, port: &str) {
    let listener = TcpListener::bind(format!("{}:{}", ip, port)).await.unwrap();
    println!("running as server on {}:{}", ip, port);

    let connections: Vec<Connection> = vec![];

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
            let connection = new_connection;
            let mut received_buffer = vec![0; 1024];
            let mut last_message: NetworkMessage =
                NetworkMessage::new("".to_string(), MessageType::Other);

            // In a loop, read data from the socket and write the data back.
            loop {
                let new_received =
                    match wait_for_client_message(&mut stream, &mut received_buffer, &connection)
                        .await
                    {
                        Ok(message) => message,
                        Err(err) => match err.kind() {
                            ErrorKind::ConnectionAborted | ErrorKind::ConnectionReset => {
                                println!(
                                    "connection aborted on {}:{}",
                                    connection.ip, connection.port
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
                    handle_new_message(&new_received, &connection, &mut stream)
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
        MessageType::SendRoomList => {}
        MessageType::JoinRoom => {}
        MessageType::Other => {}
    }

    if message.text == "hello there".to_string() {
        send_message(
            &NetworkMessage::new("general kenobi".to_string(), MessageType::Other),
            stream,
        )
        .await
        .unwrap();
    }
    return Ok(());
}
