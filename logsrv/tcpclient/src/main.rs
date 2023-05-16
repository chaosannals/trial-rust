use std::{io, str, io::Write, io::Read};
use std::net::{TcpStream};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:33333")?;

    let mut buf = [0u8; 128];
    loop {
        io::stdout().write_all(b"send: ")?;
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?; // 空输入会导致接收方读不到。TODO 禁止空输入
        let d = input.trim().as_bytes();
        stream.write(&d)?;
        let n = stream.read(&mut buf[0..128])?;
        io::stdout().write_all(&buf[0..n])?;
    }
}
