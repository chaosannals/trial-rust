mod entities;

use futures::executor::block_on;
use sea_orm::{
    ConnectionTrait,
    Database,
    DbBackend,
    DbErr,
    Statement,
    ActiveValue,
    ModelTrait,
    EntityTrait,
    ColumnTrait,
    QueryFilter,
    ActiveModelTrait,
};
use entities::{prelude::*, *};

/// MySQL	mysql://root:root@localhost:3306
/// PostgreSQL	postgres://root:root@localhost:5432
/// SQLite (in file)	sqlite:./sqlite.db?mode=rwc
/// SQLite (in memory)	sqlite::memory:
const DATABASE_URL: &str = "sqlite:./test.db";
const DB_NAME: &str = "bakeries_db"; // 这个名字如果使用 Sqlite 是用不上的。

async fn run() -> Result<(), DbErr> {
    // 此处 API 的设计非常吊轨，先链接，后再进入 get_database_backend 再执行一次初始化。
    let db = Database::connect(DATABASE_URL).await?;

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

    // 插入 
    let happy_bakery = bakery::ActiveModel {
        name: ActiveValue::Set("Happy Bakery".to_owned()),
        profit_margin: ActiveValue::Set(0.0),
        ..Default::default()
    };
    let res = Bakery::insert(happy_bakery).exec(db).await?;

    // 更改
    let sad_bakery = bakery::ActiveModel {
        id: ActiveValue::Set(res.last_insert_id),
        name: ActiveValue::Set("Sad Bakery".to_owned()),
        profit_margin: ActiveValue::NotSet,
    };
    sad_bakery.update(db).await?;

    // 插入关联数据
    for chef_name in ["Jolie", "Charles", "Madeleine", "Frederic"] {
        let john = chef::ActiveModel {
            name: ActiveValue::Set(chef_name.to_owned()),
            bakery_id: ActiveValue::Set(res.last_insert_id), // a foreign key
            ..Default::default()
        };
        Chef::insert(john).exec(db).await?;
    }

    // 全部检出
    let bakeries: Vec<bakery::Model> = Bakery::find().all(db).await?;
    println!("all: {:?}", bakeries);

    // 按 ID 查找
    let sad_bakery: Option<bakery::Model> = Bakery::find_by_id(1).one(db).await?;
    println!("find: {:?}", sad_bakery);

    //带过滤条件查询
    let sad_bakery: Option<bakery::Model> = Bakery::find()
        .filter(bakery::Column::Name.eq("Sad Bakery"))
        .one(db)
        .await?;
    println!("filter: {:?}", sad_bakery);

    // 先通过 ID 获取主数据
    let la_boulangerie: bakery::Model = Bakery::find_by_id(res.last_insert_id)
        .one(db)
        .await?
        .unwrap();
    // 检出关联数据
    let chefs: Vec<chef::Model> = la_boulangerie.find_related(Chef).all(db).await?;
    println!("la: {:?}  =>  chefs: {:?}", la_boulangerie, chefs);


    // 删除，必须指定主键
    let john = chef::ActiveModel {
        id: ActiveValue::Set(1), // The primary key must be set
        ..Default::default()
    };
    john.delete(db).await?; // 不存在不会报错。

    // 可以看到主键 id:1 是没有的，是从 2 开始。
    let chefs: Vec<chef::Model> = Chef::find().all(db).await?;
    println!("chefs: {:?}", chefs);

    Ok(())
}

// async fn run_bakery() -> Result<(), DbErr> {
//     let happy_bakery = bakery::ActiveModel {
//         name: ActiveValue::Set("Happy Bakery".to_owned()),
//         profit_margin: ActiveValue::Set(0.0),
//         ..Default::default()
//     };
//     let res = Bakery::insert(happy_bakery).exec(db).await?;
//     Ok(())
// }

fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}