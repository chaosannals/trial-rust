mod apis;
mod jobs;
mod states;

use chrono::Local;

use actix_web::{get, web, App, HttpServer, Responder};
use std::{
    cell::Cell,
    sync::atomic::{AtomicUsize, Ordering},
    sync::{Arc, Mutex},
};
use lib_demo::{init_env_logger, queue::start_queue, job, task::start_task_queue};
use apis::apis_config;
use jobs::{JobData, bar_jobs::BarJobData, foo_jobs::FooJobData};
use states::AppState;
use actix_web::rt::signal;
use futures::future;

use crate::jobs::foo_jobs::FooJobEvent;


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
    let (jobs_redis, jobs_redis_handle) = job::start_queue().await;
    let (task_sender, task_handle) = start_task_queue();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(task_sender.clone()))
            .app_data(web::Data::new(jobs_redis.clone()))
            .app_data(web::Data::new(jobs.clone()))
            .app_data(web::Data::new(bar_jobs.clone()))
            .app_data(web::Data::new(foo_jobs.clone()))
            .app_data(web::Data::new(data.clone()))
            .configure(apis_config)
    })
    .workers(4)
    .bind(("127.0.0.1", 44321))?
    .run()
    .await;

    log::info!("HTTP 服务回收");

    // let _ = future::try_join(server, worker).await;

    jobs_redis_handle.abort();
    let _ = jobs_redis_handle.await;

    log::info!("jobs_redis_handle 服务回收");

    bar_jobs_handle.abort();
    let _ = bar_jobs_handle.await;

    log::info!("bar_jobs_handle 服务回收");

    foo_jobs_handle.abort();
    let _ = foo_jobs_handle.await;

    log::info!("foo_jobs_handle 服务回收");

    jobs_handle.abort();
    let _ = jobs_handle.await;

    log::info!("jobs_handle 服务回收");

    task_handle.abort();
    let _ = task_handle.await;

    log::info!("task_handle 服务回收");

    Ok(())
}
