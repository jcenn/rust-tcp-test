use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::network_message::NetworkMessage;

pub async fn send_message(message: &NetworkMessage, stream: &mut TcpStream) {
    let json: String = serde_json::to_string(message).unwrap();
    match stream.write(json.as_bytes()).await {
        Ok(_) => {}
        Err(_) => {
            panic!("couldn't send message: {:#?}", message);
        }
    }
}
