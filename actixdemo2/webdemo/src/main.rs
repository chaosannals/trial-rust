mod apis;
mod jobs;
mod states;

use chrono::Local;

use actix_web::{get, web, App, HttpServer, Responder};
use std::{
    cell::Cell,
    sync::atomic::{AtomicUsize, Ordering},
    sync::Arc,
};
use lib_demo::{init_env_logger, queue::start_queue, job};
use apis::apis_config;
use jobs::{JobData, bar_jobs::BarJobData, foo_jobs::FooJobData};
use states::AppState;
use actix_web::rt::signal;
use futures::future;


#[actix_web::main]
async fn main() -> std::io::Result<()> {   
    init_env_logger();
    let now = Local::now().naive_local();
    log::info!("now: {:?}", now);

    let data = AppState {
        // local_count: Cell::new(0),
        global_count: Arc::new(AtomicUsize::new(0)),
    };

    let (jobs, jobs_handle) = start_queue::<JobData>();
    let (bar_jobs, bar_jobs_handle) = start_queue::<BarJobData>();
    let (foo_jobs, foo_jobs_handle) = start_queue::<FooJobData>();
    let (jobs_redis, jobs_monitor) = job::start_queue().await;

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(jobs_redis.clone()))
            .app_data(web::Data::new(jobs.clone()))
            .app_data(web::Data::new(bar_jobs.clone()))
            .app_data(web::Data::new(foo_jobs.clone()))
            .app_data(web::Data::new(data.clone()))
            .configure(apis_config)
    })
    .workers(4)
    .bind(("127.0.0.1", 44321))?
    .run();

    let worker = jobs_monitor.run_with_signal(signal::ctrl_c());

    let _ = future::try_join(server, worker).await;

    bar_jobs_handle.abort();
    let _ = bar_jobs_handle.await;

    foo_jobs_handle.abort();
    let _ = foo_jobs_handle.await;

    jobs_handle.abort();
    let _ = jobs_handle.await;

    Ok(())
}
