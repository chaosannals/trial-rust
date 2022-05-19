use std::net::UdpSocket;
use std::{io, str};

pub struct LogClient<'a> {
    host: &'a str,
    port: u16,
    sock: UdpSocket,
}

impl<'a> LogClient<'a> {
    pub fn new(host: &'a str, port: u16) -> std::io::Result<LogClient> {
        let target = format!("{0}:{1}", host, port);
        let sock = UdpSocket::bind("127.0.0.1:22221")?;
        sock.connect(target)?;
        Ok(LogClient {
            host: host,
            port: port,
            sock: sock,
        })
    }

    pub fn send(&self, data: &[u8]) -> std::io::Result<usize> {
        self.sock.send(data)
    }

    pub fn recv(&self) -> std::io::Result<()> {
        let mut buffer = [0u8; 512];
        self.sock.recv_from(&mut buffer)?;
        println!("recv: {}", str::from_utf8(&buffer).expect("eeeee"));
        Ok(())
    }
}