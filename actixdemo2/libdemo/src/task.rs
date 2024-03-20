use std::{
    sync::mpsc::{
        channel, sync_channel, Receiver, SyncSender, Sender, TryRecvError
    }, time::Duration
};
use actix_web::{
    rt::{
        task::JoinHandle,
        spawn,
        time::sleep,
    }
};

use serde::{Deserialize, Serialize};

// 如果要使用，应该把数据结构分离下，以下把几个结构合并了。但是功能其实有点不相同。
// JobTaskEvent 加了 JSON 解析是为了给接口参数复用，实际使用最好拆开。
// send 在拆开后封装成一个函数，不要让使用者去生成 结果 通道。

#[derive(Debug)]
pub struct JobTaskResult {
    pub id: i32,
    pub is_ok: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JobTaskEvent {
    pub id: i32,
    #[serde(skip)]
    pub sender: Option<Sender<JobTaskResult>>,
}

// pub struct JobTask(pub dyn Fn() + Send + Sync);

pub fn start_task_queue() -> (SyncSender<JobTaskEvent>, JoinHandle<()>) {
    let (sender, receiver) = sync_channel::<JobTaskEvent>(1);//信号量限数
    // let (sender, receiver) = channel::<JobTaskEvent>(); // 信号量不限数

    let handle = spawn(async move {
        loop {
            // 不阻塞
            match receiver.try_recv() {
                Ok(r) => {
                    log::info!("[JobTask] recv {:?}", r);
                    if let Some(s) = r.sender {
                        match s.send(JobTaskResult {
                            id: r.id,
                            is_ok: true,
                        }) {
                            Ok(r2) => {
                                log::info!("[JobTask] r2 {:?}", r2);
                            }
                            Err(e2) => {
                                log::info!("[JobTask] e2 {:?}", e2);
                            }
                        }
                    } else {
                        log::info!("[JobTask] result sender empty.");
                    }
                }
                Err(e) => {
                    match e {
                        TryRecvError::Empty => {
                            sleep(Duration::from_millis(1000)).await
                        }
                        _ => {
                            log::info!("[JobTask] err: {:?}", e);
                        }
                    }
                }
            }

            // 阻塞 这个会导致主进程 无法接收关闭命令，同线程下的其他协程 功能也被阻塞。
            // match receiver.recv() {
            //     Ok(r) => {
            //         log::info!("[JobTask] recv {:?}", r);
            //         if let Some(s) = r.sender {
            //             match s.send(JobTaskResult {
            //                 id: r.id,
            //                 is_ok: true,
            //             }) {
            //                 Ok(r2) => {
            //                     log::info!("[JobTask] r2 {:?}", r2);
            //                 }
            //                 Err(e2) => {
            //                     log::info!("[JobTask] e2 {:?}", e2);
            //                 }
            //             }
            //         } else {
            //             log::info!("[JobTask] result sender empty.");
            //         }
            //     },
            //     Err(e) => {
            //         log::info!("[JobTask] err: {:?}", e);
            //     }
            // }
        }
    });

    (sender, handle)
}