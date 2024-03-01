use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use actix_web::{
    rt,
    rt::task::JoinHandle,
};

use std::time::Duration;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct JobItem {
    pub message: String,
}

pub type JobsQueue = Arc<Mutex<VecDeque<JobItem>>>;

pub fn start_queue() -> (JobsQueue, JoinHandle<()>) {
    let queue = JobsQueue::default();
    // 不使用 CancellationToken 导致需要手动关闭开启 backend . 一般服务器不会关闭
    // tokio 有 CancellationToken 使用会使得框架固定位 tokio 后端
    // TODO 找到 actix 的 通用版本 CancellationToken
    // let cache_sweep_cancel = CancellationToken::new();

    (
        Arc::clone(&queue),
        rt::spawn(async  move {
            loop {
                let mut is_idle = false;
                if let Ok(mut q) = queue.try_lock() {
                    log::info!("worker lock");
                    if let Some(job) = q.pop_back() {
                        log::info!("job lock");
                    } else {
                        is_idle = true;
                    }
                }
                if is_idle {
                    log::info!("not job, sleep.");
                    rt::time::sleep(Duration::from_millis(10000)).await
                }
            }
        })
    )
}

pub trait RteJob {
    fn start(&mut self, item: JobItem);
}

impl RteJob for JobsQueue {
    fn start(&mut self, item: JobItem) {
        if let Ok(mut q) = self.try_lock() {
            q.push_back(item);
        }
    }
}