use std::sync::Arc;
use crate::AppState;
use crate::jobs::{JobsQueue, JobData, JobAction, JobActionArc, bar_jobs::{BarJobsQueue, BarJobData}};
use actix_web::{web:: {Data, Json, ServiceConfig, route}, Responder};
use lib_demo::job::JobRedisData;
use lib_demo::queue::JobsQueueTrait;
use async_trait::async_trait;
use apalis::{prelude::*, redis::{RedisStorage, connect}};

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

async fn into_redis_jobs(
    job: Json<JobRedisData>,
    storage: Data<RedisStorage<JobRedisData>>,
) -> impl Responder {
    let storage = &*storage.into_inner();
    let mut storage = storage.clone();
    let res = storage.push(job.into_inner()).await;
    "Ok"
}

pub fn apis_queue_config(cfg: &mut ServiceConfig) {
    cfg
        .route("/into_queue", route().to(into_queue))
        .route("/into_bar_queue", route().to(into_bar_queue))
        .route("/into_redis_jobs", route().to(into_redis_jobs));
}