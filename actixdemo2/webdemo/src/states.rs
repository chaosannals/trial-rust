
use std::{
    cell::Cell,
    sync::atomic::AtomicUsize,
    sync::Arc,
};

#[derive(Default, Clone)]
pub struct AppState {
    // pub local_count: Cell<usize>, // worker 绑定。// 这东西与多个 web::Data 使用时，会报错。
    pub global_count: Arc<AtomicUsize>, // 全局。
}