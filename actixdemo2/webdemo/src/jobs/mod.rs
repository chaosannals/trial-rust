pub mod bar_jobs;
pub mod foo_jobs;

use serde::{Deserialize, Serialize,};
use actix_web::{web::Data};
use lib_demo::queue::{JobDataTrait, JobsQueueType, JobsQueueTrait};
use crate::states::AppState;

#[derive(Default, Deserialize, Serialize, Clone)]
pub struct JobData {
    pub tag: String,
    pub message: String,
    #[serde(skip)]
    pub app: Data<AppState>,
}
pub type JobsQueue = JobsQueueType<JobData>;

impl JobDataTrait for JobData {
    async fn run(&self) {
        log::info!("jobData run {} msg: {} app: {:?}", self.tag, self.message, self.app);
    }
}