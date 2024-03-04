use actix_web::{
    get,
    post,
    web,
    rt::time,
    HttpResponse,
    Responder,
    Result,
    Error,
};
use awc::Client;
use crate::app::errors::ApiJsonError;

pub async fn get_rust_official() -> Result<impl Responder, ApiJsonError> {
    let client = Client::default();

    match client
        .get("https://www.rust-lang.org")    // <- Create request builder
        .insert_header(("User-Agent", "Actix-web"))
        .send()                             // <- Send http request
        .await
        {
            Ok(mut r) => {
                println!("Response: {:?}", r);        // <- server http response
                match r.body().await {
                    Ok(content) => {
                        println!("content: {:?}", content);
                        Ok(HttpResponse::Ok().body(content))
                    }
                    Err(e) => {
                        // e.into()
                        Err(ApiJsonError { content: format!("{:?}", e).to_string() })
                    }
                }
            },
            Err(e) => {
                // e.into()
                Err(ApiJsonError { content: format!("{:?}", e).to_string() })
            }
        }
}

pub async fn get_baidu() -> Result<impl Responder, ApiJsonError> {
    let client = Client::default();

    match client
        .get("https://www.baidu.com")    // <- Create request builder
        .insert_header(("User-Agent", "Actix-web"))
        .send()                             // <- Send http request
        .await
        {
            Ok(mut r) => {
                println!("Response: {:?}", r);        // <- server http response
                match r.body().await {
                    Ok(content) => {
                        println!("content: {:?}", content);
                        Ok(HttpResponse::Ok().body(content))
                    }
                    Err(e) => {
                        // e.into()
                        Err(ApiJsonError { content: format!("{:?}", e).to_string() })
                    }
                }
            },
            Err(e) => {
                // e.into()
                Err(ApiJsonError { content: format!("{:?}", e).to_string() })
            }
        }
}

pub async fn hmac_md5() -> Result<impl Responder, ApiJsonError> {
    
    Ok(HttpResponse::Ok())
}

pub fn do_config(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/get_rust_official", web::route().to(get_rust_official))
        .route("/get_baidu", web::route().to(get_baidu))
        .route("/hmac_md5", web::route().to(hmac_md5));
}