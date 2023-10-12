use std::time::{SystemTime};
use polodb_core::{ Database, Error, bson::{doc, Document} };

fn main() -> Result<(), Error> {
    let db_path = "demo.db";
    let db = Database::open_file(db_path)?;

    let mut session = db.start_session()?;
    session.start_transaction(None)?;

    // 强类型
    let collection = db.collection::<Document>("books");

    let docs = vec![
        doc! { "title": "TT 1984", "author": "George Orwell" },
        doc! { "title": "TT Animal Farm", "author": "George Orwell" },
        doc! { "title": "TT The Great Gatsby", "author": "F. Scott Fitzgerald" },
    ];

    collection.insert_many_with_session(docs, &mut session)?;

    let t = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(r) => r.as_secs(),
        Err(_e) => 123
    };
    
    if (t % 2) == 0 {
        session.abort_transaction()?;
        println!("回滚事务 {:?}", t);
    } else {
        session.commit_transaction()?;
        println!("提交事务 {:?}", t);
    }   

    Ok(())
}
