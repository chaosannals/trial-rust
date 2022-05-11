use clap::Clap;
use rand::prelude::*;

const CHARSET: &'static str = "1234567890_qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM";

#[derive(Clap)]
struct Opts {
    #[clap(short='l', long="length", default_value="16")]
    length: usize,
}

fn main() {
    let opts: Opts = Opts::parse();
    let charlen = CHARSET.len();
    let mut rng = thread_rng();
    for _ in 0..opts.length {
        let n = rng.gen_range(0..charlen);
        let c = CHARSET.chars().nth(n);
        print!("{0}", c.unwrap());
    }
}
