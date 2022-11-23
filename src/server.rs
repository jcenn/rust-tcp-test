use tokio::net::TcpListener;

use crate::common::{send_message, wait_for_message, MessageType, NetworkMessage};

pub async fn run(ip: &str, port: &str) {
    let listener = TcpListener::bind(format!("{}:{}", ip, port)).await.unwrap();
    println!("running as server on {}:{}", ip, port);

    loop {
        let (mut stream, addr) = listener.accept().await.unwrap();
        println!("new connection from {addr}");
        tokio::spawn(async move {
            let mut received_buffer = vec![0; 1024];

            let mut last_message: NetworkMessage =
                NetworkMessage::new("".to_string(), MessageType::Other);
            // In a loop, read data from the socket and write the data back.
            loop {
                let new_received = wait_for_message(&mut stream, &mut received_buffer).await;
                if new_received.text != last_message.text {
                    if new_received.text == "hello there".to_string() {
                        send_message(
                            &NetworkMessage::new("general kenobi".to_string(), MessageType::Other),
                            &mut stream,
                        )
                        .await
                        .unwrap();
                    }
                    last_message = new_received;
                }
            }
        });
    }
}
