use std::net::UdpSocket;
use std::{io, str};
extern crate logpack;
use logpack::LogPacker;

mod client;

fn main() -> std::io::Result<()> {
    let packer = LogPacker::new(&[0u8; 32], &[0u8; 32]);
    let client = client::LogClient::new("127.0.0.1", 22222)?;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let p = packer.log_pack("fn", input.trim().as_bytes());
        client.send(p)?;
        client.recv()?;
    }
}
