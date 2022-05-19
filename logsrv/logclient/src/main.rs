use std::net::UdpSocket;
use std::{io, str};

fn main() -> std::io::Result<()> {
    let sock = UdpSocket::bind("127.0.0.1:22221")?;
    sock.connect("127.0.0.1:22222")?;

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        sock.send(input.trim().as_bytes())?;

        let mut buffer = [0u8; 1500];
        sock.recv_from(&mut buffer)?;

        println!("recv: {}", str::from_utf8(&buffer).expect("eeeee"));
    }
}
