pub mod bar_jobs;
pub mod foo_jobs;

use std::sync::Arc;
use serde::{Deserialize, Serialize,};
use actix_web::{web::Data};
use lib_demo::queue::{JobDataTrait, JobsQueueType, JobsQueueTrait};
use crate::states::AppState;

pub trait JobAction {
    fn act(&self);
}

#[derive(Default)]
pub struct JobActionNothingToDo;

impl JobAction for JobActionNothingToDo {
    fn act(&self) {
        log::info!("JobActionNothingToDo");
    }
}

#[derive(Clone)]
pub struct JobActionArc(pub Arc<dyn JobAction + Send + Sync>);
impl Default for JobActionArc {
    fn default() -> JobActionArc {
        JobActionArc(Arc::new(JobActionNothingToDo {}))
    }
}

// 这里因为 复用 lib-demo 的代码，多了一层，实际改用这种 dyn trait 形式，可以少一层。
// 外层 start_queue 通过泛型，每种类型独立一个函数，缺点是启动需要每种一套 启动和回收代码。
// 这种 JobAction 的（可以独立外层），共用一个协程，利用 dyn trait 挂载不同类型实例。
#[derive(Default, Deserialize, Serialize, Clone)]
pub struct JobData {
    #[serde(skip)]
    pub action: JobActionArc,
}

pub type JobsQueue = JobsQueueType<JobData>;

impl JobDataTrait for JobData {
    async fn run(&self) {
        self.action.0.act();
    }
}