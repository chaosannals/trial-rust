use async_std::task;
use sqlx::mysql::{MySqlConnectOptions, MySqlSslMode};
use sqlx::{ConnectOptions, Executor, Error};

async fn init() -> Result<(), Error> {
    let mut conn = MySqlConnectOptions::new()
        .ssl_mode(MySqlSslMode::Required)
        .host("localhost")
        .username("root")
        .password("password")
        .database("trail")
        .connect()
        .await?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS t_test (
            `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
            `name` VARCHAR(50),
            PRIMARY KEY (`id`)
        )  ENGINE=INNODB DEFAULT CHARSET=utf8mb4 COLLATE = utf8mb4_unicode_ci;",
    )
    .await?;
    Ok(())
}

async fn run() {
    let s = match init().await {
        Ok(i) => format!("Ok {:?}", i),
        Err(e) => format!("Err {:?}", e),
    };
    println!("{0}", s)
}

fn main() {
    task::block_on(run())
}
