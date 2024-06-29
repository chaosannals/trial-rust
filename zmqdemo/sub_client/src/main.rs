use anyhow::Result;
use zeromq::{Socket, SocketRecv, SubSocket};
use helpers::{main, PUB_PORT, SUB_NAME};

#[main]
async fn main() -> Result<()> {
    let mut socket = SubSocket::new();
    let link = format!("tcp://127.0.0.1:{}", PUB_PORT);
    println!("connect: {}", link);
    socket.connect(&link).await?;
    socket.subscribe(SUB_NAME).await?;

    // PUB 服务器端必须先监听，不然会一直挂载但是不报错（Windows）。
    loop {
        let recv = socket.recv().await?;
        println!("recv: {:?}", recv);
    }
}