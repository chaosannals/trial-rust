use anyhow::Result;
// 客户端 ReqSocket 服务器是 RepSocket 容易写错。
use zeromq::{Socket, SocketSend, SocketRecv, ReqSocket};
use helpers::{main, REQ_PORT};

#[main]
async fn main() -> Result<()> {
    let mut socket = ReqSocket::new();
    let link = format!("tcp://127.0.0.1:{}", REQ_PORT);
    println!("connect: {}", link);
    socket.connect(&link).await?;

    for i in 0..10u32 {
        let msg = format!("Hello: {}", i);
        println!("send: {}", msg);
        socket.send(msg.into()).await?;
        let resp = socket.recv().await?;
        println!("recv: {:?}", resp);
    }
    Ok(())
}
