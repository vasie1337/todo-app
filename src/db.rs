use rusqlite::{Connection, Result as SqliteResult};
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub text: String,
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
    
    pub fn insert_task(&self, req: &CreateTaskRequest) -> SqliteResult<i64> {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        
        let conn = self.0.lock().unwrap();
        conn.execute(
            "INSERT INTO tasks (
                text, 
                completed, 
                created_at, 
                updated_at
            ) VALUES (?1, ?2, ?3, ?4)",
            (
                &req.text,
                false,
                current_time,
                current_time
            ),
        )?;
        
        Ok(conn.last_insert_rowid())
    }
}