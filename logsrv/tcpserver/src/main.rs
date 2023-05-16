use std::io::prelude::*;
use std::io::{stdout};
use std::thread::{spawn};
use std::net::{TcpListener, TcpStream};

fn main() -> std::io::Result<()> {
    let address = "0.0.0.0:33333";
    let listener = TcpListener::bind(address)?;
    //listener.set_nonblocking(true).expect("Cannot set non-blocking");

    for stream in listener.incoming() {
        // 此处是同步的，标准库没有异步库，需要使用第三方。
        // 此处读写最好另外起线程或者协程，防止阻塞。
        match stream {
            Ok(mut s) => {
                spawn(move || {
                    let b = &mut [0; 128];
                    loop {
                        let n = s.read(b).expect("read error");
                        // stdout().write_all(&b[0..n]).expect("write error"); // std 写入无效
                        s.write(b"linked").expect("read error");
                        // stdout().write_all(b"end").expect("write error"); // std 写入无效
                    }
                });
            }
            Err(_e) => {
                stdout().write_all(b"error")?;
            }
        }
    }
    Ok(())
}
