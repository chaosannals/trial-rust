use std::fs;
use std::path::Path;

// 需要通过 警告级别才能输出，TODO 优化显示，不要警告。。
macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}\n", format!($($tokens)*))
    }
}

fn main() {
    let assets_dir = Path::new("res");
    let out_dir = std::env::var("OUT_DIR").unwrap();
    p!("out: {:?}", out_dir);
    let profile = std::env::var("PROFILE").unwrap();
    p!("profile: {:?}", profile);
    let mut target_dir = Path::new(&out_dir);
    loop {
        match target_dir.file_name() {
            Some(n) => {
                if profile.eq(n.to_str().unwrap()) {
                    break;
                }
            }
            None => {
                break;
            }
        }
        target_dir = target_dir.parent().unwrap();
    }
    p!("target dir: {:?}", target_dir);

    let dest_path = target_dir.join("res");
    p!("dest path: {:?}", dest_path);
    
    // 创建目标目录
    fs::create_dir_all(&dest_path).unwrap();
    
    // 复制所有文件
    for entry in fs::read_dir(&assets_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let dest_file = dest_path.join(path.file_name().unwrap());
            fs::copy(&path, &dest_file).unwrap();
        }
    }
}