use actix_web::{error};
use derive_more::{Display, Error};
use serde::{
    Serialize,
    // Deserialize,
};
// use std::error::Error;

#[derive(Debug, Display, Error)]
#[display(fmt = "api error: {}", content)]
pub struct ApiParamError {
    pub content: &'static str,
}
// 使用 ResponseError 的默认实现
impl error::ResponseError for ApiParamError {}

//========================================================

#[derive(Debug, Display, Error)]
#[display(fmt = "{}", content)]
pub struct ApiJsonError {
    pub content: String,
}
// 使用 ResponseError 的默认实现
impl error::ResponseError for ApiJsonError {}
impl ApiJsonError {
    pub fn new<T: Serialize>(m: &T) -> ApiJsonError {
        match serde_json::to_string(&m) {
            Ok(r) => ApiJsonError {
                content: r,
            },
            Err(e) => ApiJsonError {
                content: format!("{{ code: -1, message: \"json encode error: {:?} \" }}", e),
            },
        }
    }
}