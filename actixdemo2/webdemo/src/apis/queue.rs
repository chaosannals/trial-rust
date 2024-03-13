use crate::AppState;
use crate::jobs::{JobsQueue, JobData, bar_jobs::{BarJobsQueue, BarJobData}};
use actix_web::{web:: {Data, Json, ServiceConfig, route}, Responder};
use lib_demo::queue::JobsQueueTrait;

async fn into_queue(queue: Data<JobsQueue>, param: Json<JobData>) -> impl Responder {
    queue.into_queue(param.into_inner());
    "Ok"
}

async fn into_bar_queue(queue: Data<BarJobsQueue>, param: Json<BarJobData>) -> impl Responder {
    queue.into_queue(param.into_inner());
    "Ok"
}

pub fn apis_queue_config(cfg: &mut ServiceConfig) {
    cfg
        .route("/into_queue", route().to(into_queue))
        .route("/into_bar_queue", route().to(into_bar_queue));
}