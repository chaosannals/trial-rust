use std::fs::read_dir;
use std::path::{Path, PathBuf};

#[allow(dead_code)]
pub fn get_conf_file(dir: &Path) -> Vec<PathBuf> {
    let mut result = Vec::new();
    if dir.is_dir() {
        for entry in read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                let p = get_conf_file(path.as_path());
                result.extend(p.into_iter());
            } else if path.is_file() {
                result.push(path);
            }
        }
    }
    return result;
}

#[allow(dead_code)]
pub fn get_vhosts() -> Vec<String> {
    let ep = std::env::args().nth(0).unwrap();
    let dp = Path::new(&ep).parent().unwrap();
    let mut dir = PathBuf::from(dp);
    dir.push("conf.d");
    let mut result = Vec::new();
    for p in get_conf_file(&dir) {
        let n = p.file_name().unwrap().to_str().unwrap();
        result.push(String::from(n));
    }
    return result;
}
