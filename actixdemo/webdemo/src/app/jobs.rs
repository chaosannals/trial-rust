use actix_web::{
    web,
    Result,
    Error,
    Responder,
    HttpResponse,
};

use crate::jobs::rt_ephemeral_jobs::{RteJob, JobsQueue, JobItem};

pub async fn add(rte_jobs: web::Data<JobsQueue>)-> Result<impl Responder> {
    let job = JobItem {
        message: "aaaa".to_owned(),
    };
    rte_jobs.lock().unwrap().push_back(job.clone());
    // rte_jobs.start(job.clone());
    Ok(web::Json(job))
}

pub fn do_config(cfg: & mut web::ServiceConfig) {
    cfg
    .route("/add", web::route().to(add));
}