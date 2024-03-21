
use std::{
    sync::atomic::AtomicUsize,
    sync::{Arc},
};

#[derive(Default, Debug, Clone)]
pub struct AppState {
    pub global_count: Arc<AtomicUsize>, // 全局。
}