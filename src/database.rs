pub mod database {
    use rusqlite::{Connection, Result};
    use std::fmt;
    use serde::{Serialize, Deserialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Person {
        id: i32,
        name: String,
        data: Option<Vec<u8>>,
    }

    impl fmt::Display for Person {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.name)
        }
    }

    pub fn dbtest() -> Result<Vec<Person>> {
        let conn = Connection::open("test.db")?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS person (
                id   INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                data BLOB
            )",
            (), // empty list of parameters.
        )?;
        let me = Person {
            id: 0,
            name: "Steven".to_string(),
            data: None,
        };
        conn.execute(
            "INSERT INTO person (name, data) VALUES (?1, ?2)",
            (&me.name, &me.data),
        )?;

        let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
        let person_iter = stmt.query_map([], |row| {
            Ok(Person {
                id: row.get(0)?,
                name: row.get(1)?,
                data: row.get(2)?,
            })
        })?;
        
        person_iter.collect()
        // Ok("none".to_string())
    }
}
