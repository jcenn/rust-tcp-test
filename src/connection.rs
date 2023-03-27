use std::net::IpAddr;

use serde_derive::{Deserialize, Serialize};
use tokio::net::TcpStream;

static mut LAST_ID: i32 = 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Connection {
    pub ip: IpAddr,
    pub port: u16,
    pub id: i32,
}

impl Connection {
    pub fn new(ip: IpAddr, port: u16) -> Self {
        unsafe {
            //TODO: figure out a better way to generate connection id
            LAST_ID += 1;
            Self {
                ip: ip,
                port: port,
                id: LAST_ID,
            }
        }
    }
}
