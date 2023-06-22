pub mod user;
use crate::database::user::User;

use rusqlite::{Connection, Result};

#[axum::async_trait]
pub trait Database {
    async fn get_users(&self) -> Vec<User>;
}

pub async fn dbtest() -> Result<Vec<User>> {
    let conn: Connection = Connection::open("test.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS User (
            id   INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            data BLOB
        )",
        (),
    )?;
    let me = User {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };
    conn.execute(
        "INSERT INTO User (name, data) VALUES (?1, ?2)",
        (&me.name, &me.data),
    )?;

    let mut stmt = conn.prepare("SELECT id, name, data FROM User")?;
    let user_iter = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    user_iter.collect()
    // Ok("none".to_string())
}

pub struct MockDatabase;

#[axum::async_trait]
impl Database for MockDatabase {
    async fn get_users(&self) -> Vec<User> {
        vec![
            User {
                id: 0,
                name: "Hello".to_string(),
                data: None,
            },
            User {
                id: 1,
                name: "Goofy".to_string(),
                data: None,
            },
        ]
    }
}

pub async fn get_users(database: &impl Database) -> Vec<User> {
    database.get_users().await
}