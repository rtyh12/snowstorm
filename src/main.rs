use axum::{routing::get, Router, Extension, extract::State};
use std::{net::SocketAddr, sync::Arc};
use Iterator;
mod database;

use crate::database::{Database, MockDatabase};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let database_connection = Arc::new(MockDatabase {});

    let app = Router::new()
        .route("/", get(root))
        .route("/users", get(users))
        .with_state(database_connection);

    let addr = SocketAddr::from(([192, 168, 0, 202], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub async fn users(State(database_connection): State<Arc<MockDatabase>>) -> String {
    let test = database::get_users(database_connection).await;
    serde_json::to_string_pretty(&test).unwrap()
    // "a".to_string()
}

// async fn users_db() -> String {
//     let database_connection = MockDatabase {};
//     let test = database::get_users(&database_connection).await;
//     serde_json::to_string_pretty(&test).unwrap()
//     // "a".to_string()
// }

async fn root() -> &'static str {
    "Hello, Worldyyy!"
}
