pub struct MockDatabase;

use super::*;

#[axum::async_trait]
impl Database for MockDatabase {
    async fn get_posts(&self) -> Vec<database_models::Post> {
        vec![
            database_models::Post {
                id: 0,
                name: "Hello".to_string(),
                data: None,
            },
            database_models::Post {
                id: 1,
                name: "Goofy".to_string(),
                data: None,
            },
        ]
    }
}
