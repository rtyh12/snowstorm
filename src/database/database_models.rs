use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub name: String,
    pub data: Option<Vec<u8>>,
}