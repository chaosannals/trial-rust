use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use actix_web::{
    rt::{
        task::JoinHandle,
        spawn,
        time::sleep,
    }
};
use serde::{Deserialize, Serialize,};


pub type JobsQueueType<T: Default> = Arc<Mutex<VecDeque<T>>>;
pub trait JobsQueueTrait<T :Default> {
    fn get_job(&self) -> Option<T>;
    fn into_queue(&self, job: T);
}
pub trait JobDataTrait {
    async fn run(&self);
}
impl <T: Default> JobsQueueTrait<T> for JobsQueueType<T>  {
    fn get_job(& self) -> Option<T> {
        if let Ok(mut q) = self.try_lock() {
            q.pop_back()
        } else {
            None
        }
    }
    fn into_queue(&self, job: T) {
        if let Ok(mut q) = self.try_lock() {
            q.push_back(job)
        }
    }
}

pub fn start_queue<T:Default + JobDataTrait  + 'static>() -> (JobsQueueType<T>,JoinHandle<()>) { 
    let queue = JobsQueueType::<T>::default();
    let result = Arc::clone(&queue);
    let handle = spawn(async move {
        log::info!("start queue.");
        loop {
            let mut is_idle = false;
            if let Some(job) = queue.get_job() {
                job.run().await;
            } else {
                is_idle = true;
            }

            if is_idle {
                sleep(Duration::from_millis(1000)).await
            }
        }
    });

    (result, handle)
}

