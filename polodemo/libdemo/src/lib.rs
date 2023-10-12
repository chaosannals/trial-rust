use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    pub title: String,
    pub author: String,
}
