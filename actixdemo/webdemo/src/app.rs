pub mod hello;
mod streams;

// 流请求，依赖这个扩展
use futures::StreamExt;

#[path="errors.rs"]
mod errors;

#[path="guards.rs"]
mod guards;

use actix_web::{web, guard, HttpResponse, Responder, Error, Result};
use errors::{ApiParamError, ApiJsonError};
use serde::{Serialize, Deserialize};
use core::{result};

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

async fn guard_test() -> impl Responder {
    HttpResponse::Ok()
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LoginParam {
    app_key: String,
    app_secret: String,
}

async fn login(param: web::Json<LoginParam>) -> Result<impl Responder> {
    Ok(web::Json(param))
}

// 流请求
async fn by_stream(mut body: web::Payload) -> result::Result<HttpResponse, Error> {
    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        let item = item?;
        log::info!("Chunk: {:?}", &item);
        bytes.extend_from_slice(&item);
    }
    Ok(HttpResponse::Ok().finish())
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
    )
    .route("/login", web::route().to(login))
    .route("/error_respond", web::get().to(error_respond))
    .route("/error_json", web::get().to(error_json))
    .route(
        "/guard_test",
        web::route()
        .guard(guard::Not(guard::Put()))
        .guard(guards::ContentTypeHeader)
        .guard(guard::All(guard::Post()).and(guard::Header("content-type", "application/json")))
        .to(guard_test)
    )
    .route("/by_stream", web::route().to(by_stream))
    .route("/stream_sse", web::route().to(streams::sse));
}

#[cfg(test)]
mod tests {
    use super::*;


}