extern crate rand;

include!("ip.rs");

use actix_web::{get, web, App, HttpServer, Responder};
use rand::distributions::Uniform;
use rand::prelude::*;

fn make_set(length: i32) -> Vec<i32> {
    let mut r = vec![];
    let u = Uniform::new(0, 10000000);
    for _ in 0..length {
        r.push(u.sample(&mut rand::thread_rng()));
    }
    return r;
}

#[get("/{id}/{name}/index.html")]
async fn user_index(info: web::Path<(u32, String)>) -> impl Responder {
    let a: Vec<i32> = make_set(10000);
    format!("Hello {0}! id:{1} => {2:?}", info.1, info.0, a)
}

#[get("/ip.html")]
async fn index() -> impl Responder {
    let ip = get_wan_ip().await;
    format!("Ip {0}", ip.unwrap())
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(user_index).service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
