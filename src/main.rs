use std::{
    env,
    io::{stdin, Read, Write},
    net::{TcpListener, TcpStream},
    str::from_utf8,
    thread,
};
mod client;
mod server;
mod common;
mod request;
mod response;
mod network_message;

static IP_ADDRESS: &str = "127.0.0.1";
static PORT: &str = "3333";

#[tokio::main]
async fn main() {
    let config = env::args().nth(1).unwrap().replace("--", "");
    println!("args: {}", config);

    if config == "server" {
        server::run(IP_ADDRESS, PORT).await;
    } else if config == "client" {
        client::run(IP_ADDRESS, PORT).await;
    }
}
