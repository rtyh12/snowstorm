use serde::{Serialize, Deserialize};

use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: PostId,
    pub user_id: UserId,
    pub content: String,
    pub parent_id: Option<PostId>,
    pub likes: i64,
    pub comments_ids: Vec<PostId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: PostId,
    pub name: String,
    pub bio: Option<String>,
}