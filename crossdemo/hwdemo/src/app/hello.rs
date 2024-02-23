use std::sync::Mutex;
use std::sync::Arc;
use actix_web::{get, post, web, HttpResponse, Responder};

pub struct AppStateWithCounter {
    pub counter: Mutex<i32>, // <- Mutex 线程安全
}

#[get("")]
async fn hello(data: web::Data<AppStateWithCounter>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    HttpResponse::Ok().body(format!("Request number: {counter}"))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub fn make_config(cfg: & mut web::ServiceConfig,counter: &web::Data<AppStateWithCounter>) {
    log::info!("on make_config");

    cfg.app_data(counter.clone())
        .service(hello)
        .service(echo)
        .route("/hey", web::get().to(manual_hello));
}