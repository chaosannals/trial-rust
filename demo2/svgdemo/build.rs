use std::{
    fs,
    path::{ Path}
};

// 需要通过 警告级别才能输出
macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}\n", format!($($tokens)*))
    }
}

fn main () {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    p!("out: {:?}", out_dir);
    let profile = std::env::var("PROFILE").unwrap();
    p!("profile: {:?}", profile);
    let mut target_dir = Path::new(&out_dir);

    // 当前命令行目录
    let cwd = std::env::current_dir().unwrap();
    p!("cwd: {:?}", cwd);

    // target 目录下的 exe 文件路径
    let exe_path = std::env::current_exe().unwrap();
    p!("exe_path: {:?}", exe_path);

    let svg_path = cwd.join("demo.svg");
    p!("svg_path: {:?}", svg_path);
}