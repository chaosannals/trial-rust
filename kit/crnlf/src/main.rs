use std::{io::{self, Write, Read, SeekFrom, Seek}, fs, path};
use clap::Parser;

#[derive(Parser, Debug)]
struct Opts {
    #[arg(short='t', long="target", default_value=".")]
    target: String,
    #[arg(short='s', long="suffix", default_value="txt")]
    suffix: String,
    #[arg(short='m', long="mode", default_value="crlf2lf")]
    mode: String,
    #[arg(short='e', long="is_print_error", value_parser, default_value_t=false)]
    is_print_error: bool,
}

fn replace_file(p: path::PathBuf, src: &str, dist: &str) -> io::Result<()> {
    let mut fr = fs::OpenOptions::new()
        .read(true)
        .open(p.clone())?;
    let mut content = String::new();
    let s = fr.read_to_string(&mut content)?;
    let nc = content.replace(src, dist);

    let mut fw = fs::OpenOptions::new()
        .write(true)
        .append(false)
        .truncate(true)
        .open(p.clone())?;
    fw.seek(SeekFrom::Start(0))?;
    fw.write_all(nc.as_bytes())?;
    fw.flush()?;
    println!("replace: {} {}", s, p.display());
    Ok(())
}

fn replace_dir(dirpath: path::PathBuf, src: &str, dist: &str, is_print_error: bool, suffix: &str) -> io::Result<u32> {
    let mut count = 0;
    for entry in fs::read_dir(dirpath)? {
        let p = entry?.path();
        if p.is_dir() && !p.ends_with(".") {
            count += replace_dir(p.clone(), src, dist, is_print_error, suffix)?;
        } else if p.is_file() {
            let b = match p.extension() {
                Some(s) => s.eq_ignore_ascii_case(suffix),
                None => false,
            };
            if b {
                match replace_file(p.clone(), src, dist) {
                    Ok(_) => count += 1,
                    Err(e) => {
                        if is_print_error {
                            println!("error: {} {:?}", p.display(), e)
                        }
                    },
                }
            } else {
                println!("skip: {}", p.display())
            }
        }
    }
    Ok(count)
}


fn main() {
    let opts: Opts = Opts::parse();
    let tdir = path::PathBuf::from(opts.target);
    let is_print_error = opts.is_print_error;
    let suffix = opts.suffix.as_str();
    let r = match opts.mode.as_str() {
        "crlf2lf" => replace_dir(tdir, "\r\n", "\n", is_print_error, suffix),
        "crlf2cr" => replace_dir(tdir, "\r\n", "\r", is_print_error, suffix),
        "lf2crlf" => replace_dir(tdir, "\n", "\r\n", is_print_error, suffix),
        "lf2cr" => replace_dir(tdir, "\n", "\r", is_print_error, suffix),
        "cr2crlf" => replace_dir(tdir, "\r", "\r\n", is_print_error, suffix),
        "cr2lf" => replace_dir(tdir, "\r", "\n", is_print_error, suffix),
        _ => Ok({ println!("unknown mode"); 0 }),
    };
    // println!("suffix {}", suffix);
    match r {
        Err(e) => println!("error {:?}", e),
        Ok(count) => println!("final, replace count: {}", count),
    }
}