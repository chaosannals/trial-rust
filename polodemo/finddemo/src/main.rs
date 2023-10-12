use libdemo::Book;
use polodb_core::{ Database, Error, bson::{doc, Regex} };

fn main() -> Result<(), Error> {
    let db_path = "demo.db";
    let db = Database::open_file(db_path)?;

    let collection = db.collection::<Book>("books");

    // 全部，无条件，
    let books = collection.find(None)?;
    for book in books {
        println!("all name: {:?}", book);
    }

    // 条件全匹配
    let books2 = collection.find(doc! {
        "author": "George Orwell",
        // 直接接下一条字段是 AND
    })?;

    for book in books2 {
        println!("2 name: {:?}", book);
    }

    // Or
    let books3 = collection.find(doc! {
        "$or": [
            {"title": "The Grapes of Wrath",},
            {"title": "To Kill a Mockingbird",},
        ],
    })?;
    for book in books3 {
        println!("3 name: {:?}", book);
    }

    // 条件语句的写法基本上和 mongodb 类似，操作符都是 $ 开头。
    // 用 JSON 写一个复合的语法树，来表示条件逻辑。
    // $eq(等于)  $gt（大于）  $gte（大于等于）  $lt（小于） $lte（小于等于） $ne（不等于）
    // $in(是否在数组内)  $nin(not in)  $regex（正则，而且需要特殊结构, 官方文档有错误）

    let books4 = collection.find(doc! {
        "$or": [
            {
                "age": { "$eq": 18 },
            },
            {
                "author": { 
                    "$regex": Regex{
                        pattern: "\\w*George\\w*".to_string(),
                        options: "i".to_string(), // 模式i忽略大小写
                    },
                },
            },
        ],
    })?;
    for book in books4 {
        println!("4 name: {:?}", book);
    }

    Ok(())
}
