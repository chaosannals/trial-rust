use clap::Parser;
use rand::prelude::*;

const CHARSET_UPPERCASE_LETTER: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const CHARSET_LOWERCASE_LETTER: &'static str = "abcdefghijklmnopqrstuvwxyz";
const CHARSET_DIGIT: &'static str = "0123456789";
const CHARSET_SYMBOL: &'static str = "~!@#$%^&*_-+";

#[derive(Parser, Debug)]
struct Opts {
    #[arg(short='l', long="length", default_value="16")]
    length: usize,

    #[arg(short='c', long="charset", default_value="Lld", help="L:[A-Z] l:[a-z] d:[0-9] s:[~!@#$%^&*_-+]")]
    charset: String,

    #[arg(short='a', long="append", default_value="_", help="append charset.")]
    append_charset: String,
}

fn main() {
    let opts: Opts = Opts::parse();
    let mut charset = String::from(opts.append_charset);
    for c in opts.charset.chars() {
        match c {
            'L' => charset.push_str(CHARSET_UPPERCASE_LETTER),
            'l' => charset.push_str(CHARSET_LOWERCASE_LETTER),
            'd' => charset.push_str(CHARSET_DIGIT),
            's' => charset.push_str(CHARSET_SYMBOL),
            _ => {},
        };
    }
    let charlen = charset.len();
    let mut rng = thread_rng();
    for _ in 0..opts.length {
        let n = rng.gen_range(0..charlen);
        let c = charset.chars().nth(n);
        print!("{0}", c.unwrap());
    }
}
