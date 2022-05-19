use std::net::UdpSocket;
use std::{thread, time, io};

pub struct LogServer<'a> {
    host: &'a str,
    port: u16,
}

impl<'a> LogServer<'a> {
    pub fn new(host: &'a str, port: u16) -> LogServer<'a> {
        LogServer {
            host: host,
            port: port,
        }
    }

    pub fn serve(&self) -> std::io::Result<()> {
        let addr = format!("{0}:{1}", self.host, self.port);
        println!("bind: {0}", addr);
        let sock = UdpSocket::bind(addr)?;
        sock.set_nonblocking(true)?;
        let mut buf = [0;512];
        loop {
            match sock.recv_from(&mut buf) {
                Ok((n, src)) => {
                    // println!("1 {} {} {:?}", n, src, buf);
                    let r = &mut buf[0..n];
                    r.reverse();
                    println!("2 {} {} {:?}", n, src, r);
                    sock.send_to(&r, &src)?;
                },
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // println!("wait_for_fd()");
                    thread::sleep(time::Duration::from_millis(1));
                }
                Err(e) => panic!("encountered IO error: {}", e),
            }
        };
    }
}