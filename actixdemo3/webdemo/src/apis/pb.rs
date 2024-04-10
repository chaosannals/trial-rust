use actix_protobuf::*;
use actix_web::{web, Result, HttpResponse};
// use anyhow::Result;

pub mod echopb {
    tonic::include_proto!("/grpc.examples.echo");
}

// 限制有点死。居然不能用 anyhow
async fn pb_echo(msg: ProtoBuf<echopb::EchoRequest>) -> Result<HttpResponse> {
    print!("echo");
    HttpResponse::Ok().protobuf(echopb::EchoResponse {
        message: msg.0.message,
    })
}

pub fn apis_pb_config(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/echo", web::route().to(pb_echo));
}