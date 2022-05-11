use async_std::task;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use sqlx::{ConnectOptions, Executor};
use std::str::FromStr;

async fn create_tables() -> Result<(), sqlx::Error> {
    let mut conn = SqliteConnectOptions::from_str("sqlite:trial.db")?
        .journal_mode(SqliteJournalMode::Wal)
        .create_if_missing(true)
        .connect()
        .await?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS t_test (
        id INTEGER PRIMARY KEY ASC AUTOINCREMENT,
        name VARCHAR(50) 
    )",
    )
    .await?;
    Ok(())
}

async fn do_it() {
    let s = match create_tables().await {
        Ok(i) => format!("Ok {:?}", i),
        Err(e) => format!("Err {:?}", e),
    };
    println!("{0}", s)
}

fn main() {
    task::block_on(do_it());
}
