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
    DatabaseConnection,
    MockDatabase,//mock
    FromQueryResult, // Query 0.引入依赖
    sea_query::{
        Alias,
        Expr,
        JoinType,
        Order,
        Query,
        // QueryBuilder,
        MysqlQueryBuilder, // stmt.to_string
        PostgresQueryBuilder, // 
        SqliteQueryBuilder
    }, // Query 0.引入依赖
};
use entities::{prelude::*, *};

use serde_json::json;

/// Query 1. 定义查询结果结构
#[derive(FromQueryResult, Debug)]
struct ChefNameResult {
    name: String,
}

/// MySQL	mysql://root:root@localhost:3306
/// MySQL	mysql://root:root@localhost:3306/db_name
/// PostgreSQL	postgres://root:root@localhost:5432
/// SQLite (in file)	sqlite:./sqlite.db?mode=rwc
/// SQLite (in memory)	sqlite::memory:
const DATABASE_URL: &str = "sqlite:./test.db";
const DB_NAME: &str = "bakeries_db"; // 这个名字如果使用 Sqlite 是用不上的。

async fn run() -> Result<(), DbErr> {
    // 此处 API 的设计非常吊轨，先链接，后再进入 get_database_backend 再执行一次初始化。
    let db = Database::connect(DATABASE_URL).await?;

    // 构建链接时执行创建数据库语句。get_database_backend 应该是可以重复调用。
    let builder = db.get_database_backend();
    // let db = &match builder {
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

    /// Query 2. SELECT 的字段名
    let column = (chef::Entity, Alias::new("name")); // 这种写法类型丢了。。。
    let mut stmt = Query::select();
    stmt.column(column.clone()) // Use `expr_as` instead of `column` if renaming is necessary
        .from(chef::Entity)
        .join(
            JoinType::Join,
            bakery::Entity,
            Expr::tbl(chef::Entity, Alias::new("bakery_id"))
                .equals(bakery::Entity, Alias::new("id")),
        )
        .order_by(column, Order::Asc);
    
    let chef = ChefNameResult::find_by_statement(builder.build(&stmt))
        .all(db)
        .await?;
    println!("Query: chef: {:?}", chef);
    println!("MySQL: {}", stmt.to_string(MysqlQueryBuilder));
    println!("Postgres: {}", stmt.to_string(PostgresQueryBuilder));
    println!("Sqlite: {}", stmt.to_string(SqliteQueryBuilder));

    // 完全由 JSON 构建， 没有缺省，必须JSON字段齐全。
    let fjsonid = res.last_insert_id + 1;
    let fjson = bakery::ActiveModel::from_json(json!({
        "id": fjsonid,
        "name": "AAA from JSON",
        "profit_margin": 0.0,
    }))?;
    Bakery::insert(fjson).exec(db).await?;

    // Model 已经构建后通过 json 加载 修改 字段。
    let mut fjson = bakery::ActiveModel {
        profit_margin: ActiveValue::Set(0.0),
        ..Default::default() // 这里可以使用默认填充。
    };
    // seaorm 无法使用 serde 的 默认值。
    fjson.set_from_json(json!({
        "id": fjsonid + 1,
        "name": "BBB set from JSON",
        "profit_margin": 1.0,
    }))?;

    Bakery::insert(fjson).exec(db).await?;

    Ok(())
}

// 提供测试数据 mock
async fn run_mock() -> Result<(), DbErr> {
    // 示例太老了，DatabaseBackend::MySql 被改成了 DbBackend::MySql，官方示例没改。。。
    let db: &DatabaseConnection = &MockDatabase::new(DbBackend::MySql)
        .append_query_results(vec![
            // First query result
            vec![bakery::Model {
                id: 1,
                name: "Happy Bakery".to_owned(),
                profit_margin: 0.0,
            }],
            // Second query result
            vec![
                bakery::Model {
                    id: 1,
                    name: "Happy Bakery".to_owned(),
                    profit_margin: 0.0,
                },
                bakery::Model {
                    id: 2,
                    name: "Sad Bakery".to_owned(),
                    profit_margin: 100.0,
                },
                bakery::Model {
                    id: 3,
                    name: "La Boulangerie".to_owned(),
                    profit_margin: 17.89,
                },
            ],
        ])
        .append_query_results(vec![
            // Third query result
            vec![
                chef::Model {
                    id: 1,
                    name: "Jolie".to_owned(),
                    contact_details: None,
                    bakery_id: 3,
                },
                chef::Model {
                    id: 2,
                    name: "Charles".to_owned(),
                    contact_details: None,
                    bakery_id: 3,
                },
                chef::Model {
                    id: 3,
                    name: "Madeleine".to_owned(),
                    contact_details: None,
                    bakery_id: 3,
                },
                chef::Model {
                    id: 4,
                    name: "Frederic".to_owned(),
                    contact_details: None,
                    bakery_id: 3,
                },
            ]
        ])
        .into_connection();

    let happy_bakery: Option<bakery::Model> = Bakery::find().one(db).await?;
    println!("mock: {:?}", happy_bakery);
    
    Ok(())
}

fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }

    if let Err(err) = block_on(run_mock()) {
        panic!("{}", err);
    }
}