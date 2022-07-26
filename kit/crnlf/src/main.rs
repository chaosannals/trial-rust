use std::{io::{self, Write, Read}, fs, path};
use clap::Parser;

#[derive(Parser, Debug)]
struct Opts {
    #[clap(short='t', long="target", default_value=".")]
    target: String,
    #[clap(short='m', long="mode", default_value="crlf2lf")]
    mode: String,
}

fn replace_file(p: path::PathBuf, src: &str, dist: &str) -> io::Result<()> {
    let mut f = fs::File::open(p.clone())?;
    let mut content = String::new();
    let s = f.read_to_string(&mut content)?;
    let nc = content.replace(src, dist);
    f.write_all(nc.as_bytes())?;
    println!("replace: {} {}", s, p.display());
    Ok(())
}

fn replace_dir(dirpath: path::PathBuf, src: &str, dist: &str) -> io::Result<()> {
    for entry in fs::read_dir(dirpath)? {
        let p = entry?.path();
        //println!("p: {}", p.display());
        if p.is_dir() && !p.ends_with(".") {
            replace_dir(p, src, dist)?;
        } else if p.is_file() {
            match replace_file(p, src, dist) {
                Ok(_) => (),
                Err(e) => println!("error: {:?}", e),
            }
        }
    }
    Ok(())
}


fn main() {
    let opts: Opts = Opts::parse();
    let tdir = path::PathBuf::from(opts.target);
    let r = match opts.mode.as_str() {
        "crlf2lf" => replace_dir(tdir, "\r\n", "\n"),
        "crlf2cr" => replace_dir(tdir, "\r\n", "\r"),
        "lf2crlf" => replace_dir(tdir, "\n", "\r\n"),
        "lf2cr" => replace_dir(tdir, "\n", "\r"),
        "cr2crlf" => replace_dir(tdir, "\r", "\r\n"),
        "cr2lf" => replace_dir(tdir, "\r", "\n"),
        _ => Ok({ println!("unknown mode"); }),
    };
    match r {
        Err(e) => println!("error {:?}", e),
        Ok(_) => println!("final."),
    }
}
