use std::net::UdpSocket;
use std::{thread, time, io};

fn main() -> std::io::Result<()> {
    let sock = UdpSocket::bind("127.0.0.1:22222")?;
    sock.set_nonblocking(true)?;
    let mut buf = [0;1000];
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
