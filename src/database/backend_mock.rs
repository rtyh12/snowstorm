pub struct MockDatabase;

use super::*;

#[axum::async_trait]
impl Database for MockDatabase {
    async fn get_posts(&self, for_user_id: UserId) -> Vec<database_models::Post> {
        vec![
            database_models::Post {
                id: 0,
                user_id: 0,
                content: "Hello".to_string(),
                parent_id: None,
                likes: 10,
                comments_ids: vec![1],
            },
            database_models::Post {
                id: 1,
                user_id: 1,
                content: "ratio".to_string(),
                parent_id: Some(0),
                likes: 100,
                comments_ids: vec![],
            },
        ]
    }

    async fn get_posts_by_user(&self, user_id: UserId) -> Vec<database_models::Post> {
        vec![
            database_models::Post {
                id: 0,
                user_id: 0,
                content: "same".to_string(),
                parent_id: None,
                likes: 10,
                comments_ids: vec![1],
            },
            database_models::Post {
                id: 1,
                user_id: 0,
                content: "user".to_string(),
                parent_id: Some(0),
                likes: 100,
                comments_ids: vec![],
            },
        ]
    }

    async fn get_comments_for_post(&self, post_id: PostId) -> Vec<database_models::Post> {
        vec![
            database_models::Post {
                id: 0,
                user_id: 0,
                content: "same".to_string(),
                parent_id: None,
                likes: 10,
                comments_ids: vec![1],
            },
            database_models::Post {
                id: 1,
                user_id: 0,
                content: "user".to_string(),
                parent_id: Some(0),
                likes: 100,
                comments_ids: vec![],
            },
        ]
    }

    async fn get_user(&self, with_id: UserId) -> User {
        User { id: 100, name: "obama".to_string(), bio: Some("hola".to_string()) }
    }

    async fn make_post(
        &self,
        user_id: UserId,
        content: String,
        parent_id: Option<PostId>,
        auth_key: String,
    ) -> bool {
        return false; 
    }
}
