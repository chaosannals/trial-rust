use actix_web::{
    web,
    Result,
    Responder,
    HttpResponse,
};
use sea_orm::*;
use sea_orm::ActiveModelTrait;
use sea_orm::entity::prelude::*;
use chrono::{Local, DateTime};

use crate::app::AppState;
use crate::entities::user::ActiveModel;

async fn index(path: web::Path<(u32, String)>) -> Result<String> {
    let (user_id, friend) = path.into_inner();
    Ok(format!("Welcome {}, user_id {}!", friend, user_id))
}

async fn add(data: web::Data<AppState>) -> Result<impl Responder> {
    let conn = &data.conn;
    let obj = ActiveModel {
        age: Set(1),
        nickname: Set("aaaa".to_owned()),
        create_at: Set(Local::now().naive_local()),
        ..Default::default()
    }.save(conn)
    .await;

    Ok(
        web::Json(obj.unwrap().nickname.unwrap())
    )
}

pub fn do_config(cfg: & mut web::ServiceConfig) {
    cfg
        .route("/{user_id}/{friend}", web::route().to(index))
        .route("/add", web::route().to(add));
}