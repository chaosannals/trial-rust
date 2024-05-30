
use rusty_leveldb::{DB, Options, CompressorId, compressor::NoneCompressor};
use anyhow::Result;

fn main() -> Result<()> {
    let mut opts = Options::default();
    opts.compressor = NoneCompressor::ID;
    let mut db = DB::open("word.ldb", opts)?;
    let w = "key";
    let s = "value";
    db.put(w.as_bytes(), s.as_bytes()).unwrap();
    Ok(())
}
