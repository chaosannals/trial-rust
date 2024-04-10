mod pb;

use std::{
    cell::Cell,
    sync::atomic::{AtomicUsize, Ordering},
    sync::Arc,
};
use actix_web::{get, web, Responder};
use serde::{Serialize, Deserialize};

use crate::states::AppState;

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

#[derive(Serialize)]
struct AddOneResult {
    global_count: usize,
}

#[get("/add_one")]
async fn add_one(data: web::Data<AppState>) -> impl Responder {
    data.global_count.fetch_add(1, Ordering::Relaxed);

    // let local_count = data.local_count.get();
    // data.local_count.set(local_count + 1);

    let r = AddOneResult {
        global_count: data.global_count.load(Ordering::Relaxed),
        // data.local_count.get()
    };
    web::Json(r)
}


pub fn apis_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(show_count)
        .service(add_one)
        .service(
            web::scope("/pb")
            .configure(pb::apis_pb_config)
        );
}

#[cfg(test)]
mod tests {
    use actix_web::{http::header::ContentType, test, App};
    use super::*;

    #[actix_web::test]
    async fn test_show_count() {
        let state = AppState {
            global_count: Arc::new(AtomicUsize::new(0)),
        };
        let app = test::init_service(App::new().app_data(web::Data::new(state.clone())).service(show_count)).await;
        let req = test::TestRequest::get().uri("/show_count")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_add_one() {
        let state = AppState {
            global_count: Arc::new(AtomicUsize::new(0)),
        };
        let app = test::init_service(App::new().app_data(web::Data::new(state.clone())).service(add_one)).await;
        let req = test::TestRequest::get().uri("/add_one")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}