use anyhow::Result;
// 客户端 ReqSocket 服务器是 RepSocket 容易写错。
use zeromq::{Socket, SocketRecv, SocketSend, RepSocket};
use std::time::Duration;
// use std::convert::TryInto;
use helpers::{main, sleep, REQ_PORT};

#[main]
async fn main() -> Result<()> {
    let mut socket = RepSocket::new();
    let link = format!("tcp://0.0.0.0:{}", REQ_PORT);
    socket.bind(&link).await?;
    println!("bind: {}", &link);

    loop {
        // let rep: String = socket.recv().await?.try_into()?;
        match socket.recv().await {
            Ok(r) => {
                println!("{:?}", r);
                socket.send("aaa".into()).await?;
            },
            Err(err) =>{
                println!("{:?}", err);
                sleep(Duration::from_secs(4)).await;
            },
        };
    }
}
