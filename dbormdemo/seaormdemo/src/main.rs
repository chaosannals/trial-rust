use futures::executor::block_on;
use sea_orm::{ConnectionTrait, Database, DbBackend, DbErr, Statement};

/// MySQL	mysql://root:root@localhost:3306
/// PostgreSQL	postgres://root:root@localhost:5432
/// SQLite (in file)	sqlite:./sqlite.db?mode=rwc
/// SQLite (in memory)	sqlite::memory:
const DATABASE_URL: &str = "sqlite:./test.db";
const DB_NAME: &str = "bakeries_db";

async fn run() -> Result<(), DbErr> {
    // 构建链接时执行创建数据库语句。
    let db = &match db.get_database_backend() {
        DbBackend::MySql => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE IF NOT EXISTS `{}`;", DB_NAME),
            )).await?;
            let url = format!("{}/{}", DATABASE_URL, DB_NAME);
            Database::connect(&url).await?
        }
        DbBackend::Postgres => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("DROP DATABASE IF EXISTS \"{}\";", DB_NAME),
            )).await?;
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE \"{}\";", DB_NAME),
            )).await?;
            let url = format!("{}/{}", DATABASE_URL, DB_NAME);
            Database::connect(&url).await?
        }
        DbBackend::Sqlite => {
            // TODO ，自动创建 数据库文件。
            db
        }, 
    };

    Ok(())
}

fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}