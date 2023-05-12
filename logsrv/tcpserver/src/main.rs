use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() -> std::io::Result<()> {
    let address = "0.0.0.0:33333";
    let listener = TcpListener::bind(address)?;
    for stream in listener.incoming() {
        // 此处是同步的，标准库没有异步库，需要使用第三方。
        // 此处读写最好另外起线程或者协程，防止阻塞。
        stream?.write(b"linked")?;
    }
    Ok(())
}
