use std::time::Duration;
use std::env;
use sea_orm::*;

#[actix::main]
async fn start() -> std::io::Result<()>  {
    // get env vars
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        // .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path("my_schema".into());

    // 官方也没有解决多种 Error 类型的问题，使用 unwrap 绕过了。
    let db = Database::connect(opt).await.unwrap();
    Ok(())
}

fn main()  {
    println!("Hello, SeaORM");
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}")
    }
}
