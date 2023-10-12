use libdemo::Book;
use polodb_core::{ Database, Error, bson::{doc} };

fn main() -> Result<(), Error> {
    let db_path = "demo.db";
    let db = Database::open_file(db_path)?;

    let collection = db.collection::<Book>("books");

    // 条件可以随便写，没有的字段也不会报错
    let deleted_result = collection.delete_many(doc!{
        "$or": [
            {"_id": 123,},
            {"age": { "$gt": 123 }, },
        ],
    })?;

    println!("result: {:?}", deleted_result);
    
    Ok(())
}
