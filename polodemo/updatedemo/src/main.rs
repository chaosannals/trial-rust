// use libdemo::Book;
use polodb_core::{ Database, Error, bson::{doc, Document} };

fn main() -> Result<(), Error> {
    let db_path = "demo.db";
    let db = Database::open_file(db_path)?;

    let collection = db.collection::<Document>("books");

    collection.update_many(doc! { // 条件
        // "_id": "c2" // TODO 这个 _id 和 mongodb 一样是内置的，还是示例的
        "$or": [
            { "title": "1983", },
            { "title": "c33", },
        ],
    }, doc! { // 修改操作
        "$set": doc! {
            "title": "c33",
        },
        "$set": doc! {
            "newfield": 1234, // 不存在的字段好像是无效的。
        },
    })?;

    // $inc 自增
    // $min $max $mul $rename $set $unset

    Ok(())
}
