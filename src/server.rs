use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use crate::common::NetworkMessage;

pub async fn run(ip: &str, port: &str) {
    let listener = TcpListener::bind(format!("{}:{}", ip, port)).await.unwrap();
    println!("running as server on {}:{}", ip, port);

    loop {
        let (mut stream, addr) = listener.accept().await.unwrap();
        println!("new connection from {addr}");
        tokio::spawn(async move {
            let mut received_buffer = vec![0; 1024];
            // let mut line = String::with_capacity(512);
            // let mut received_count = 0;

            // In a loop, read data from the socket and write the data back.
            let mut last_message: NetworkMessage = NetworkMessage::new("".to_string());
            loop {
                // let message = NetworkMessage::new("hello there".to_string());
                // send_message(&message, &mut stream).await;
                let new_received = wait_for_message(&mut received_buffer, &mut stream).await;
                if new_received.text != last_message.text {
                    if new_received.text == "hello there".to_string() {
                        send_message(
                            &NetworkMessage::new("general kenobi".to_string()),
                            &mut stream,
                        )
                        .await;
                    }
                    last_message = new_received;
                }

                // let n = stream
                //     .read(&mut buf)
                //     .await
                //     .expect("failed to read data from socket");

                // println!("received: {:#}", std::str::from_utf8(&buf).unwrap());
                // if n == 0 {
                //     return;
                // }
                // received_count += 1;

                // if received_count == 1 {
                //     stream
                //         .write("ganske pikk".as_bytes())
                //         .await
                //         .expect("failed to write data to socket");
                // }
            }
        });
    }
}

async fn send_message(message: &NetworkMessage, stream: &mut TcpStream) {
    println!("sent a message");
    let json: String = serde_json::to_string(message).unwrap();
    stream.write(json.as_bytes()).await.unwrap();
}

async fn wait_for_message(buffer: &mut [u8], stream: &mut TcpStream) -> NetworkMessage {
    let n = stream
        .read(buffer)
        .await
        .expect("failed to read data from socket");
    let json = std::str::from_utf8(buffer).unwrap().replace('\0', "");
    let message = serde_json::from_str::<NetworkMessage>(json.as_str()).unwrap();
    println!("received {:?}", message.text);
    return message;
}
