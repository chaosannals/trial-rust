// use diesel::connection::DefaultLoadingMode;
// use diesel::prelude::*;
// // use std::io::Error;
// use diesel::result::Error;
// use diesel::r2d2::{self, ConnectionManager};
// use diesel::SqliteConnection;

// pub type Pool = r2d2::Pool<ConnectionManager>;

// table! {
//     users (id) {
//         id -> Integer,
//         name -> Text,
//         address -> Text,
//         date_created -> Text,
//     }
// }

// use serde::{Deserialize, Serialize};

// #[derive(Debug, Serialize, Deserialize, Queryable)]
// pub struct User {
//     pub id: i32,
//     pub name: String,
//     pub address: String,
//     pub date_create: String,
// }

// extern crate r2d2_foodb;

// TODO 
fn main() {
    println!("Hello, Diesel");

    // let manager = r2d2_foodb::FooConnectionManager::new("file://./test.db");
    // let pool = r2d2::Pool::builder()
    //     .max_size(15)
    //     .build(manager)
    //     .unwrap();

    // let database_url = "file://./test.db";
    // let database_url = std::env::var("DATABASE_URL").expect("NOT FOUND");
    // let database_pool = Pool::builder()
    //     .build(ConnectionManager::new(database_url))
    //     .unwrap();

    // let connection = "file://./test.db";
    // let iter1 = users::table.load_iter::<(i32, String), DefaultLoadingMode>(connection)?;
    // let iter1 = users::table.load_iter::<(i32, String), DefaultLoadingMode>(connection).expect("链接失败");

    // for r in iter1 {
    //     let (id, name) = r.expect("结果失败");
    //     println!("Id: {} Name: {}", id, name);
    // }

    // Ok(())
}
