//  actix-taskqueue 示例，废弃。
// #[derive(Debug, Default, Clone, Copy)]
// pub struct PlusFive(i32);
// pub struct PlusFiveResult(i32);

// #[async_trait]
// pub impl QueueConsumer<PlusFive, PlusFiveResult> for TaskWorker<PlusFive, PlusFiveResult> {
//     async fn execute(&self, task: PlusFive) -> Result<PlusFiveResult, WorkerExecuteError> {
//         let PlusFive(n) = task;
//         if n >= 5 {
//             return Ok(PlusFiveResult(n + 5));
//         } else if n > 0 {
//             return Err(WorkerExecuteError::Retryable);
//         } else {
//             return Err(WorkerExecuteError::NonRetryable);
//         }
//     }

//     fn get_queue(&self) -> Addr<TaskQueue<PlusFive>> {
//         TaskQueue::<PlusFive>::from_registry()
//     }

//     fn retry(&self, task: PlusFive) -> PlusFive {
//         let PlusFive(n) = task;
//         println!("RETRYING VALUE = {}", n);
//         PlusFive(n + 1)
//     }

//     fn drop(&self, task: PlusFive) {
//         let PlusFive(n) = task;
//         println!("DROPPED TASK WITH VALUE = {}", n);
//     }

//     fn result(&self, result: PlusFiveResult) {
//         let PlusFiveResult(n) = result;
//         println!("RESULT = {}", n);
//     }
// }