use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub number: Option<i32>, // 后加入的字段必须是Option可选的，不然在已有数据的时候会报缺少字段错误。
}
