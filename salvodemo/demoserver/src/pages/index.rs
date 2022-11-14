use salvo::prelude::*;

#[handler]
pub async fn index() -> &'static str {
    "Hello world"
}