use std::sync::Mutex;
use actix_web::{get, post, web, HttpResponse, Responder};

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex 线程安全
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

pub fn make_config() -> Box<dyn for<'a> Fn(&'a mut web::ServiceConfig)>  {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    Box::new(move | cfg: &mut web::ServiceConfig | {
        cfg.app_data(counter.clone())
        .service(hello)
        .service(echo)
        .route("/hey", web::get().to(manual_hello));
    })
}