use rusqlite::{Connection, Result as SqliteResult};
use serde::Deserialize;
use std::sync::{Arc, Mutex};

#[derive(Debug, Deserialize)]
pub struct TaskEntry {
    pub id: i64,
    pub text: String,
    pub completed: bool,
    pub created_at: i64,
    pub updated_at: i64
}

#[derive(Clone)]
pub struct DataBase(
    pub Arc<Mutex<Connection>>
);

impl DataBase {
    pub fn open(path: &str) -> SqliteResult<Self> {
        let conn = Connection::open(path)?;
        Ok(Self(Arc::new(Mutex::new(conn))))
    }

    pub fn migrate(&self) -> SqliteResult<()> {
        self.0.lock().unwrap().execute_batch(
        "CREATE TABLE IF NOT EXISTS tasks(
                id INTEGER PRIMARY KEY,
                text TEXT,
                completed BOOLEAN,
                created_at INTEGER,
                updated_at INTEGER
            )",
        )?;
        Ok(())
    }
    
}