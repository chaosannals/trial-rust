use std::sync::{Arc, Mutex, mpsc::{channel, }};
use crate::AppState;
use crate::jobs::{JobsQueue, JobData, JobAction, JobActionArc, bar_jobs::{BarJobsQueue, BarJobData}, foo_jobs::{FooJobsQueue, FooJobData}};
use actix_web::{web:: {Data, Json, ServiceConfig, route}, Responder};
use std::sync::mpsc::SyncSender;
use futures::SinkExt;
use lib_demo::job::JobRedisData;
use lib_demo::queue::JobsQueueTrait;
use async_trait::async_trait;
use apalis::{prelude::*, redis::{RedisStorage, connect}};
use lib_demo::task::{JobTaskEvent, JobTaskResult};

struct JobActionOne {
    app: Data<AppState>,
}

#[async_trait]
impl JobAction for JobActionOne {
    async fn act(&self) {
        log::info!("job into. {:?}", self.app);
    }
}

// 这个 Json 不会自动填充 add_data 的数据，param.app 是有值的，但是是默认值。
async fn into_queue(app: Data<AppState>, queue: Data<JobsQueue>, param: Json<JobData>) -> impl Responder {
    let mut job = param.into_inner();
    job.action = JobActionArc(Arc::new(JobActionOne {
        app: app,
    }));
    queue.into_queue(job);
    "Ok"
}

async fn into_bar_queue(app: Data<AppState>, queue: Data<BarJobsQueue>, param: Json<BarJobData>) -> impl Responder {
    let mut job = param.into_inner();
    job.app = app;
    queue.into_queue(job);
    "Ok"
}

async fn into_foo_queue(app: Data<AppState>, queue: Data<FooJobsQueue>, param: Json<FooJobData>) -> impl Responder {
    let mut job = param.into_inner();
    job.app = app;
    queue.into_queue(job);
    "Ok"
}

async fn into_redis_jobs(
    job: Json<JobRedisData>,
    storage: Data<RedisStorage<JobRedisData>>,
) -> impl Responder {
    let storage = &*storage.into_inner();
    let mut storage = storage.clone();
    let res = storage.push(job.into_inner()).await;
    "Ok"
}


async fn into_task_jobs(
    job: Json<JobTaskEvent>,
    sender: Data<SyncSender<JobTaskEvent>>,
) -> impl Responder {
    log::info!("into_task_jobs start.");

    let mut task = job.into_inner();
    let (result_sender, result_receiver) = channel::<JobTaskResult>();
    task.sender = Some(result_sender);
    log::info!("into_task_jobs send task");
    match sender.send(task) {
        Ok(r) => {
            log::info!("into_task_jobs recv result");
            match result_receiver.recv() {
                Ok(r) => format!("Ok: {:?}", r),
                Err(e) => format!("err2: {:?}", e)
            }
        }
        Err(e) => {
            format!("err: {:?}", e)
        }
    }
}

pub fn apis_queue_config(cfg: &mut ServiceConfig) {
    cfg
        .route("/into_queue", route().to(into_queue))
        .route("/into_bar_queue", route().to(into_bar_queue))
        .route("/into_foo_queue", route().to(into_foo_queue))
        .route("/into_redis_jobs", route().to(into_redis_jobs))
        .route("/into_task_jobs", route().to(into_task_jobs));
}