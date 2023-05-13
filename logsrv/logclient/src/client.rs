use std::net::UdpSocket;
use std::{io, str};
use logpack::LogPacker;

pub struct LogClient<'a> {
    host: &'a str,
    port: u16,
    sock: UdpSocket,
    packer: LogPacker,
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
            packer: LogPacker::new(
                &[0u8; 32],
                &[1u8; 32],
            )
        })
    }

    pub fn send(&self, data: &[u8]) -> std::io::Result<usize> {
        // let mut v = vec![];
        // v.extend_from_slice(data);
        // let r = self.packer.log_pack(v.as_slice());
        // let r = data;
        let r = self.packer.log_pack(data);
        self.sock.send(r.as_slice())
    }

    pub fn recv(&self) -> std::io::Result<()> {
        let mut buffer = [0u8; 512];
        self.sock.recv_from(&mut buffer)?;
        println!("recv: {}", str::from_utf8(&buffer).expect("eeeee"));
        Ok(())
    }
}