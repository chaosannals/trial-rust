pub mod hello;

#[path="errors.rs"]
mod errors;

use actix_web::{web, HttpResponse};
use errors::{ApiParamError, ApiJsonError};
use serde::{Serialize, Deserialize};

async fn error_respond() -> Result<&'static str, ApiParamError> {
    Err(ApiParamError { content: "test" })
}


#[derive(Serialize, Deserialize, Debug)]
pub struct DemoErrorMessage {
    code: i32,
    message:  &'static str,
}

async fn error_json() -> Result<&'static str, ApiJsonError> {
    let m = DemoErrorMessage {
        code: 123,
        // message: msg,
        message: "aaaaa",
    };
    Err(ApiJsonError::new(&m))
}

// this function could be located in a different module
pub fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| async { HttpResponse::Ok().body("test") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

// this function could be located in a different module
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| async { HttpResponse::Ok().body("app") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    ).route("/error_respond", web::get().to(error_respond))
    .route("/error_json", web::get().to(error_json));
}