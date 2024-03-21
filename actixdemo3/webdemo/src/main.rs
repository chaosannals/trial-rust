mod apis;
mod wsapis;
mod states;

use actix_web::{get, web, App, HttpServer, Responder};
use std::{
    cell::Cell,
    sync::atomic::{AtomicUsize, Ordering},
    sync::{Arc, Mutex},
};

use lib_demo::init_env_logger;

use crate::apis::apis_config;
use crate::wsapis::wsapis_config;
use crate::states::AppState;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    init_env_logger();
    log::info!("start");

    let state = AppState {
        global_count: Arc::new(AtomicUsize::new(0)),
    };

    let _server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .configure(apis_config)
            .configure(wsapis_config)
    })
    .workers(4)
    .bind(("127.0.0.1", 44322))?
    .run()
    .await?;

    Ok(())
}
