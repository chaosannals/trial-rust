use actix_web::{error};
use derive_more::{Display, Error};
use serde::{Serialize, Deserialize};

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
    content: String,
}
// 使用 ResponseError 的默认实现
impl error::ResponseError for ApiJsonError {}
impl ApiJsonError {
    pub fn new<T: Serialize>(m: &T) -> ApiJsonError {
        ApiJsonError {
            content: serde_json::to_string(&m).unwrap()
        }
    }
}