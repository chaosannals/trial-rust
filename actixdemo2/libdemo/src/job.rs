use std::{time::Duration};
use apalis::{prelude::*, redis::{RedisStorage, connect}, layers::tracing::TraceLayer, utils::TokioExecutor};
use serde::{Deserialize, Serialize};
use actix_web::rt::{
    task::JoinHandle,
    spawn,
    time,
    signal,
};

// 缺点这种方式依赖 序列化任务后存 redis ，所以任务项不能是闭包。。

#[derive(Debug, Deserialize, Serialize)]
pub struct JobRedisData {
    tag: String,
    message: String,
}

// 执行的任务会被存到 Redis 下  {NAME}:data 里面
impl Job for JobRedisData {
    const NAME: &'static str = "I'm a Job";
}

async fn queue_process(
    job: JobRedisData
) {
    log::info!("queue process start {:?}", job);
    time::sleep(Duration::from_millis(10000)).await;
    log::info!("queue process end {:?}", job);
}

pub async fn start_queue() -> (RedisStorage<JobRedisData>, JoinHandle<()>) {
    let redis_url = "redis://127.0.0.1:6379/1";
    let redis = connect(redis_url).await.expect("Redis 连接失败");
    let storage = RedisStorage::new(redis);

    let monitor = Monitor::<TokioExecutor>::new().register_with_count(2, {
        WorkerBuilder::new("xxxx")
            .layer(TraceLayer::new())
            .with_storage(storage.clone())
            .build_fn(queue_process)
    });
    
    let handle = spawn(async move {
        log::info!("start redis queue.");
        monitor.run_with_signal(signal::ctrl_c()).await;
    });

    (storage, handle)
}