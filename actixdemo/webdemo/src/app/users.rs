use actix_web::{
    web,
    Result,
    Error,
    Responder,
    HttpResponse,
};
use sea_orm::*;
use sea_orm::ActiveModelTrait;
use chrono::{Local};
use serde::Deserialize;

use crate::app::errors::{ApiParamError};
use crate::app::{AppState};
use crate::entities::user::{
    ActiveModel,
    Entity as User,
};

async fn index(path: web::Path<(u32, String)>) -> Result<String> {
    let (user_id, friend) = path.into_inner();
    Ok(format!("Welcome {}, user_id {}!", friend, user_id))
}

#[derive(Deserialize)]
struct UserAddParam {
    age: i32,
    nickname: String,
}

async fn add(data: web::Data<AppState>, param: web::Json<UserAddParam>) -> Result<impl Responder> {
    let conn = &data.conn;
    let obj = ActiveModel {
        age: Set(param.age),
        nickname: Set(param.nickname.to_owned()),
        create_at: Set(Local::now().naive_local()),
        ..Default::default()
    }.save(conn)
    .await;

    Ok(
        web::Json(obj.unwrap().id.unwrap())
    )
}


#[derive(Deserialize)]
struct UserFindParam {

}

async fn find(data: web::Data<AppState>, param: web::Json<UserFindParam>)  -> Result<impl Responder, ApiParamError> {
    match User::find().all(&data.conn).await {
        Ok(objs) => {
            Ok(
                web::Json(objs)
            )
        },
        Err(e) => {
            Err(ApiParamError{
                content: "eeee"
            })
        }
    }
}

pub fn do_config(cfg: & mut web::ServiceConfig) {
    cfg
        .route("/{user_id}/{friend}", web::route().to(index))
        .route("/add", web::route().to(add))
        .route("/find", web::route().to(find));
}