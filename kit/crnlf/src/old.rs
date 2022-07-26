use std::io::{self, Read, Write};
use std::fs::File;
use walkdir::WalkDir;

//WalkDir 这个库不可用，私自处理了权限异常，在里面直接panic! 了。
fn replace_dir2(dirpath: &str, src: &str, dist: &str) -> io::Result<()> {
    for entry in WalkDir::new(dirpath) {
        let entry = entry.expect("entry error");
        let path = entry.path();
        if path.ends_with(".") {
            continue;
        }
        match File::open(path) {
            Ok(mut f) => {
                let mut content = String::new();
                
                match f.read_to_string(&mut content) {
                    Ok(s) => {
                        let nc = content.replace(src, dist);
                        f.write_all(nc.as_bytes())?;
                        //println!("size: {} {}", s, nc);
                        println!("replace: {} {}", s, path.display());
                    },
                    _ => {
                        println!("read error: {}", path.display());
                        ()
                    },
                }
                
            },
            _ => {
                println!("open error: {}", path.display());
                ()
            }
        };
    }
    Ok(())
}

fn replace_dir1(dirpath: &str, src: &str, dist: &str) -> io::Result<()> {
    let r = panic::catch_unwind(|| {
        match replace_dir2(dirpath, src, dist) {
            Ok(_) => println!("ok"),
            Err(e) => println!("{:?}", e),
        }
    });
    match r {
        Ok(_) => println!("ok."),
        Err(e) => println!("{:?}", e),
    };
    Ok(())
}