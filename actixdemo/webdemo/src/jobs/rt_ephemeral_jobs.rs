use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use actix_web::{
    rt,
    rt::task::JoinHandle,
};

use std::time::Duration;

pub struct JobItem {
    message: String,
}

pub type JobsQueue = Arc<Mutex<VecDeque<JobItem>>>;

pub fn start_queue() -> (JobsQueue, JoinHandle<()>) {
    let queue = JobsQueue::default();

    (
        Arc::clone(&queue),
        rt::spawn(async {
            loop {
                if let Ok(job) = queue.try_lock() {
                    log::info!("worker do");
                    rt::time::sleep(Duration::from_millis(10000)).await
                }
            }
        })
    )
}

pub 