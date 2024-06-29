use anyhow::Result;
use std::time::Duration;
use zeromq::{Socket, SocketSend, PubSocket, ZmqMessage};
use helpers::{main, PUB_PORT, sleep, SUB_NAME};

#[main]
async fn main() -> Result<()> {
    let mut socket = PubSocket::new();
    let link = format!("tcp://127.0.0.1:{}", PUB_PORT);
    println!("bind: {}", link);
    socket.bind(&link).await?;

    // 订阅是根据消息头部
    let stocks = vec![
        "AAA".to_string(),
        "BBB".to_string(),
        "CCC".to_string(),
        SUB_NAME.to_string(),
        format!("{} 1235555", SUB_NAME)
    ];
    loop {
        for stock in &stocks {
            let msg = ZmqMessage::from(stock.clone());
            socket.send(msg).await?;
        }
        sleep(Duration::from_secs(4)).await;
    }
}