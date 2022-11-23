use std::io::Read;

use tokio::net::TcpStream;

use crate::common::{send_message, wait_for_message, MessageType, NetworkMessage};

pub async fn run(ip: &str, port: &str) {
    // let stdin = io::stdin();

    // let mut input_buf: String = String::new();
    let mut read_buf:Vec<u8> = vec![0 as u8; 1024];

    let mut stream:TcpStream = TcpStream::connect(format!("{}:{}", ip, port))
        .await
        .unwrap();

    let mut last_message: NetworkMessage = NetworkMessage::new("".to_string(), MessageType::Other);
    send_message(
        &NetworkMessage::new("hello there".to_string(), MessageType::Other),
        &mut stream,
    )
    .await
    .unwrap();

    loop {
        let new_message:NetworkMessage = wait_for_message(&mut stream, &mut read_buf).await;

        if new_message.text != last_message.text {
            last_message = new_message;
        }
    }
}
