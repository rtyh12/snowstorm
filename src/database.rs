pub mod database_models;
pub mod backend_mock;

// use crate::database::models::post::Post;

// use rusqlite::{Connection, Result};

#[axum::async_trait]
pub trait Database {
    async fn get_posts(&self) -> Vec<database_models::Post>;
}

// pub async fn dbtest() -> Result<Vec<database_models::Post>> {
//     let conn: Connection = Connection::open("test.db")?;

//     conn.execute(
//         "CREATE TABLE IF NOT EXISTS User (
//             id   INTEGER PRIMARY KEY,
//             name TEXT NOT NULL,
//             data BLOB
//         )",
//         (),
//     )?;
//     let me = database_models::Post {
//         id: 0,
//         name: "Steven".to_string(),
//         data: None,
//     };
//     conn.execute(
//         "INSERT INTO User (name, data) VALUES (?1, ?2)",
//         (&me.name, &me.data),
//     )?;

//     let mut stmt = conn.prepare("SELECT id, name, data FROM User")?;
//     let user_iter = stmt.query_map([], |row| {
//         Ok(database_models::Post {
//             id: row.get(0)?,
//             name: row.get(1)?,
//             data: row.get(2)?,
//         })
//     })?;

//     user_iter.collect()
//     // Ok("none".to_string())
// }