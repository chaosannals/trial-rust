use rusty_leveldb::{AsyncDB, Options, CompressorId, compressor::NoneCompressor};
use anyhow::Result;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let mut opts = Options::default();
    opts.compressor = NoneCompressor::ID;
    let adb = AsyncDB::new("async.ldb", opts)?;
    adb.put("Hello".as_bytes().to_owned(), "World".as_bytes().to_owned())
        .await?;
    adb.flush().await?;
    adb.close().await?;
    Ok(())
}
