use std::net::UdpSocket;
use std::io;

fn main() -> std::io::Result<()> {
    let sock = UdpSocket::bind("127.0.0.1:22222")?;
    sock.set_nonblocking(true)?;
    let mut buf = [0;10];
    let (num_bytes_read, src) = loop {
        match sock.recv_from(&mut buf) {
            Ok(n) => break n,
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                println!("1111");
                // wait_for_fd();
            }
            Err(e) => panic!("encountered IO error: {}", e),
        }
    };
    buf.reverse();
    sock.send_to(&buf, &src)?;
    Ok(())
}
