pub struct MockDatabase;

use super::*;

#[axum::async_trait]
impl Database for MockDatabase {
    async fn get_posts(&self) -> Vec<database_models::Post> {
        vec![
            database_models::Post {
                id: 0,
                user_id: 0,
                content: "Hello".to_string(),
                parent_id: None,
            },
            database_models::Post {
                id: 1,
                user_id: 1,
                content: "Woah".to_string(),
                parent_id: None,
            },
        ]
    }
}
