use libdemo::Book;
use polodb_core::{ Database, Error, bson::{Document, doc} };

fn main() -> Result<(), Error> {
    let db_path = "demo.db";
    let db = Database::open_file(db_path)?;

    // 强类型
    let collection = db.collection::<Book>("books");
    collection.insert_one(Book {
        title: "The Three-Body Problem".to_string(),
        author: "Liu Cixin".to_string(),
        ..Default::default()
    })?;
    let books = vec![
        Book {
            title: "The Grapes of Wrath".to_string(),
            author: "John Steinbeck".to_string(),
            number: Some(1),
        },
        Book {
            title: "To Kill a Mockingbird".to_string(),
            author: "Harper Lee".to_string(),
            number: Some(2),
        },
    ];
    collection.insert_many(books)?;

    // 弱类型 BSON
    let typed_collection = db.collection::<Document>("books");
    let _ = typed_collection.insert_one(doc! {
        "title": "1983", "author": "George Orwell Second"
    });
    let docs = vec![
        doc! { "title": "1984", "author": "George Orwell" },
        doc! { "title": "Animal Farm", "author": "George Orwell" },
        doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
    ];
    typed_collection.insert_many(docs)?;

    Ok(())
}
