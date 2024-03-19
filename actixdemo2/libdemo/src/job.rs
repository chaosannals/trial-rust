use std::{time::Duration, collections::HashMap};
use apalis::{prelude::*, redis::{RedisStorage, connect}, layers::tracing::TraceLayer, utils::TokioExecutor};
use serde::{Deserialize, Serialize};
use actix_web::rt::{
    task::JoinHandle,
    spawn,
    time,
    signal,
};

// use awc::Client;

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
    // awc 无法在非 actix web 之外环境被调用。
    // let handle = spawn(async {
    //     let url = "https://baidu.com";
    //     let client = Client::default();
    //     match client.get(url)
    //         .timeout(Duration::from_secs(30))
    //         .send()
    //         .await
    //         {
    //             Ok(mut r) => {
    //                 log::info!("Ok");
    //             }
    //             Err(e) => {
    //                 log::info!("err: {:?}", e);
    //             }
    //         };
    // });
    // handle.await;

    match reqwest::get("https://baidu.com")
        .await {
            Ok(r1) => {
                log::info!("r1: {:?}", r1);
                match r1.json::<HashMap<String, String>>()
                    .await {
                        Ok(r2) => {
                            log::info!("r2: {:?}", r2);
                        }
                        Err(e) => {
                            log::info!("err2: {:?}", e);
                        }
                    }
            }
            Err(e) => {
                log::info!("err1: {:?}", e);
            }
        };
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