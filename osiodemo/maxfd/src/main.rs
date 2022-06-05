use std::{env, io, fs, path};

fn sum_size(dir: &path::PathBuf) -> io::Result<u64> {
    let mut result: u64 = 0;
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            result += sum_size(&path)?;
        }
        else {
            result += fs::metadata(path)?.len();
        }
    }
    Ok(result)
}

fn fmt_size(size: u64) -> String {
    return match size {
        n @ 0 ..=1024 => format!("{}b", n),
        n @ 1025 ..=1048576 => format!("{:.2}kb", n as f64 / 1024.0),
        n @ 1048577 ..=1073741824 => format!("{:.2}mb", n as f64 / 1048576.0),
        n @ _ => format!("{:.2}gb", n as f64 / 1073741824.0),
    }
}

fn main() -> io::Result<()>{
    let args :Vec<String> = env::args().collect();
    let wkdir = if args.len() > 1 {
        args[1].clone()
    } else {
        env::current_dir()?
        .as_os_str()
        .to_str()
        .unwrap()
        .to_string()
    };
    println!("wkdir: {0}", wkdir);
    for entry in fs::read_dir(wkdir)? {
        let path = entry?.path();
        print!("{0:?} ", path);
        if path.is_dir() {
            let size = sum_size(&path)?;
            println!("dsize: {0}", fmt_size(size));
        } else {
            let size = fs::metadata(path)?.len();
            println!("fsize: {0}", fmt_size(size));
        }
    }
    Ok(())
}
