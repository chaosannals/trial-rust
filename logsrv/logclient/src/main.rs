use std::net::UdpSocket;
use std::{io, str, io::Write};
// extern crate logpack;
// use logpack::LogPacker;

mod client;

fn main() -> std::io::Result<()> {
    // let packer = LogPacker::new(&[0u8; 32], &[0u8; 32]);
    let client = client::LogClient::new("127.0.0.1", 22222)?;
    
    loop {
        io::stdout().write_all(b"send: ")?;
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let d = input.trim().as_bytes();
        // let p = packer.log_pack(d);
        client.send(d)?;
        client.recv()?;
    }
}
