use std::io::Read;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::common::NetworkMessage;

pub async fn run(ip: &str, port: &str) {
    // let stdin = io::stdin();

    // let mut input_buf: String = String::new();
    let mut read_buf = vec![0 as u8; 1024];

    let mut stream = TcpStream::connect(format!("{}:{}", ip, port))
        .await
        .unwrap();

    let mut last_message: NetworkMessage = NetworkMessage::new("".to_string());
    send_message(&NetworkMessage::new("hello there".to_string()), &mut stream).await;

    loop {
        let new_message = wait_for_message(&mut read_buf, &mut stream).await.unwrap();

        if new_message.text != last_message.text {
            last_message = new_message;
        }
    }
}

async fn wait_for_message(buffer: &mut [u8], stream: &mut TcpStream) -> Result<NetworkMessage, ()> {
    let n = stream
        .read(buffer)
        .await
        .expect("failed to read data from socket");

    let json = std::str::from_utf8(&buffer).unwrap().replace('\0', "");
    let data = serde_json::from_str::<NetworkMessage>(json.as_str())
        .expect("error while trying to parse incoming data");

    println!("received message: {:?}", data.text);
    return Ok(data);
}

async fn send_message(message: &NetworkMessage, stream: &mut TcpStream) {
    let json: String = serde_json::to_string(message).unwrap();
    stream.write(json.as_bytes()).await.unwrap();
}
