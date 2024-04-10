use anyhow::Result;
use std::collections::HashMap;
use prost::Message;

pub mod echopb {
    tonic::include_proto!("/grpc.examples.echo");
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("client start!");
    let client = reqwest::Client::new();
    let req = echopb::EchoRequest {
        message: "bbbb".to_string()
    };
    // TODO 
    let req_len = req.encoded_len();
    let mut bytes = vec![0; req_len];
    req.encode(&mut bytes)?;
    let resp = client.post("http://127.0.0.1:44322/pb/echo")
        .header("Content-Type", "application/protobuf")
        .body(bytes)
        .send()
        .await?;
    println!("{resp:#?}");
    Ok(())
}
