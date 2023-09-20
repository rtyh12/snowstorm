use axum::extract::Path;
use axum::routing::post;
use axum::{extract::State, routing::get, Router};
use database::{PostId, UserId};
use std::{net::SocketAddr, sync::Arc};

use crate::database::backend_sqlite::SQLiteDatabase;
use crate::database::backend_mock::MockDatabase;
use crate::database::Database;

mod database;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let database_connection = Arc::new(MockDatabase::new());
    // let database_connection = Arc::new(SQLiteDatabase {});

    let app = Router::new()
        .route("/", get(root))
        .route("/get_posts/:user_id", get(get_posts))
        .route("/get_posts_by_user/:user_id", get(get_posts_by_user))
        .route(
            "/get_comments_for_post/:post_id",
            get(get_comments_for_post),
        )
        .route("/get_user/:with_id", get(get_user))
        .route(
            "/make_post/:user_id/:content/:parent_id/:auth_key",
            post(make_post),
        )
        .with_state(database_connection);

    // let addr = SocketAddr::from(([192, 168, 0, 202], 3000));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8001));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub async fn get_posts(
    Path(user_id): Path<UserId>,
    State(database_connection): State<Arc<impl Database>>,
) -> String {
    let test = database_connection.get_posts(user_id).await;
    serde_json::to_string_pretty(&test).unwrap()
}

pub async fn get_posts_by_user(
    Path(user_id): Path<UserId>,
    State(database_connection): State<Arc<impl Database>>,
) -> String {
    let test = database_connection.get_posts_by_user(user_id).await;
    serde_json::to_string_pretty(&test).unwrap()
}

pub async fn get_comments_for_post(
    Path(post_id): Path<PostId>,
    State(database_connection): State<Arc<impl Database>>,
) -> String {
    let test = database_connection.get_comments_for_post(post_id).await;
    serde_json::to_string_pretty(&test).unwrap()
}

pub async fn get_user(
    Path(with_id): Path<UserId>,
    State(database_connection): State<Arc<impl Database>>,
) -> String {
    let test = database_connection.get_user(with_id).await;
    serde_json::to_string_pretty(&test).unwrap()
}

pub async fn make_post(
    Path((user_id, content, parent_id, auth_key)): Path<(UserId, String, PostId, String)>,
    State(database_connection): State<Arc<impl Database>>,
) -> String {
    let test = database_connection
        .make_post(user_id, content, Some(parent_id), auth_key)
        .await;
    serde_json::to_string_pretty(&test).unwrap()
}

async fn root() -> &'static str {
    "Hello, Worldyyy!"
}
