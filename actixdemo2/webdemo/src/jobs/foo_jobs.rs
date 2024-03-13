use serde::{Deserialize, Serialize,};
use actix_web::{web::Data};
use lib_demo::queue::{JobDataTrait, JobsQueueType, JobsQueueTrait};
use crate::states::AppState;

#[derive(Default, Deserialize, Serialize, Clone)]
pub struct FooJobData {
    pub tag: i32,
    pub message: String,
    #[serde(skip)]
    pub app: Data<AppState>,
}
pub type FooJobsQueue = JobsQueueType<FooJobData>;

impl JobDataTrait for FooJobData {
    async fn run(&self) {
        log::info!("jobData run foo");
    }
}