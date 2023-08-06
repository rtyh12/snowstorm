use axum::extract::Path;
use axum::{extract::State, routing::get, Router};
use std::{net::SocketAddr, sync::Arc};

use crate::database::backend_mock::MockDatabase;
use crate::database::Database;

mod database;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let database_connection = Arc::new(MockDatabase {});

    let app = Router::new()
        .route("/", get(root))
        .route("/get_posts/:user_id", get(get_posts))
        .with_state(database_connection);

    let addr = SocketAddr::from(([192, 168, 0, 202], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub async fn get_posts(
    Path(user_id): Path<u64>,
    State(database_connection): State<Arc<impl Database>>
) -> String {
    let test = database_connection.get_posts(user_id).await;
    serde_json::to_string_pretty(&test).unwrap()
}

async fn root() -> &'static str {
    "Hello, Worldyyy!"
}
