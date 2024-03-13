mod queue;

use std::{
    cell::Cell,
    sync::atomic::{AtomicUsize, Ordering},
    sync::Arc,
};

use actix_web::{get, web, Responder};

use crate::AppState;

use queue::apis_queue_config;

#[get("/show_count")]
async fn show_count(data: web::Data<AppState>) -> impl Responder {
    format!(
        "global_count: {}",
        data.global_count.load(Ordering::Relaxed)
    )
    // format!(
    //     "global_count: {}\nlocal_count: {}",
    //     data.global_count.load(Ordering::Relaxed),
    //     data.local_count.get()
    // )
}

#[get("/add_one")]
async fn add_one(data: web::Data<AppState>) -> impl Responder {
    data.global_count.fetch_add(1, Ordering::Relaxed);

    // let local_count = data.local_count.get();
    // data.local_count.set(local_count + 1);

    format!(
        "global_count: {}",
        data.global_count.load(Ordering::Relaxed)
    )
    // format!(
    //     "global_count: {}\nlocal_count: {}",
    //     data.global_count.load(Ordering::Relaxed),
    //     data.local_count.get()
    // )
}

pub fn apis_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(show_count)
        .service(add_one)
        .service(
            web::scope("/queue")
            .configure(apis_queue_config)
        );
}