use actix_web::{
    get,
    post,
    web,
    web::Json,
    rt::time,
    HttpResponse,
    Responder,
    Result,
    Error,
};
use awc::Client;
use crate::app::errors::ApiJsonError;
use serde::Deserialize;

use md5::Md5;
use hmac::{Hmac, Mac};
// use hex_literal::hex;

use uuid::Uuid;

use crate::app::{AppState};

// 自定义 lib 名叫 lib-demo ，但是所有的 - 都会在引用时使用 _ 替代。
use lib_demo::{get_self_exe_dir, get_self_exe_dir_path};

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
                        Err(ApiJsonError { content: format!("error (1) {:?}", e).to_string() })
                    }
                }
            },
            Err(e) => {
                // e.into()
                Err(ApiJsonError { content: format!("error (2) {:?}", e).to_string() })
            }
        }
}

#[derive(Deserialize)]
struct HmacMd5Param {
    content: String,
}

pub async fn hmac_md5(app: web::Data<AppState>, param: web::Json<HmacMd5Param>) -> Result<impl Responder, ApiJsonError> {
    let key = app.hmac_md5.as_bytes();
    match Hmac::<Md5>::new_from_slice(key) {
        Ok(mut mac) => {
            let content = param.content.as_bytes();
            mac.update(content);
            let result = mac.finalize();
            let code_bytes = result.into_bytes();
            let cs = (&code_bytes[..]).to_vec();
            let r = String::from_utf8_lossy(&cs).into_owned();
            println!("hex: {:x?}", cs);
            // let h = format!("{:x?}", cs);
            let h = hex::encode(cs);
            Ok(HttpResponse::Ok().body(h))
            // match String::from_utf8((&code_bytes[..]).to_vec()) {
            //     Ok(r) => Ok(HttpResponse::Ok().body(r)),
            //     Err(e) => Err(ApiJsonError { content: format!("{:?}", e).to_string() })
            // }
        },
        Err(e) => {
            Err(ApiJsonError { content: format!("{:?}", e).to_string() })
        }
    }
}

pub async fn make_uuid() -> Result<impl Responder, ApiJsonError> {
    let id = Uuid::new_v4();
    Ok(HttpResponse::Ok().body(id.to_string()))
}

pub async fn exe_dir() -> Result<impl Responder, ApiJsonError> {
    let the_dir = get_self_exe_dir();
    let mut env_path = get_self_exe_dir_path();
    env_path.push(".env");
    Ok(Json((the_dir, env_path)))
}

pub fn do_config(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/get_rust_official", web::route().to(get_rust_official))
        .route("/get_baidu", web::route().to(get_baidu))
        .route("/hmac_md5", web::route().to(hmac_md5))
        .route("/make_uuid", web::route().to(make_uuid))
        .route("/exe_dir", web::route().to(exe_dir));
}