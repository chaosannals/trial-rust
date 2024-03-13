use serde::{Deserialize, Serialize,};
use actix_web::{web::Data};
use lib_demo::queue::{JobDataTrait, JobsQueueType, JobsQueueTrait};
use crate::states::AppState;

#[derive(Default, Deserialize, Serialize, Clone)]
pub struct BarJobData {
    pub tag: f64,
    pub message: String,
    #[serde(skip)]
    pub app: Data<AppState>,
}
pub type BarJobsQueue = JobsQueueType<BarJobData>;

impl JobDataTrait for BarJobData {
    async fn run(&self) {
        log::info!("jobData run bar: tag: {} msg: {}", self.tag, self.message);
    }
}