use std::error::Error;
use polodb_core::{ Database, IndexModel, IndexOptions, bson::{Document, doc} };

fn main() -> Result<(), Box<dyn Error>> {
    let db_path = "demo.db";
    let db = Database::open_file(db_path)?;

    let collection = db.collection::<Document>("books");
 
    // 普通索引
    collection.create_index(IndexModel {
        keys: doc! {
            "age": 1, // 没有的字段也是随便加，通过了。
        },
        options: None,
    })?;

    // 唯一索引
    collection.create_index(IndexModel {
        keys: doc! {
            "name": 1,
        },
        options: Some(IndexOptions{
            name: Some("name_unique".to_string()),
            unique: Some(true),
            ..Default::default()
        }),
    })?;

    // 删除
    collection.drop_index("name_unique")?;

    Ok(())
}
