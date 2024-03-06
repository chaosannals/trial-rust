use std::{
    borrow::Cow, collections::HashMap, env, path::{Path, PathBuf}, sync::{Arc, RwLock}
};

pub fn get_self_exe_dir() -> String {
    let exe_path_str = env::current_exe().unwrap();
    let exe_path = Path::new(&exe_path_str);
    let exe_dir = exe_path.parent().unwrap();
    log::info!("current exe dir: {:?}", exe_dir);
    exe_dir.to_string_lossy().to_string()
}

pub fn get_self_exe_dir_path() -> PathBuf {
    let exe_path_str = env::current_exe().unwrap();
    let exe_path = Path::new(&exe_path_str);
    let exe_dir = exe_path.parent().unwrap();
    log::info!("current exe dir: {:?}", exe_dir);
    PathBuf::from(exe_dir)
}