use rusqlite::{Connection, Result as SqliteResult, Row};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct Task {
    pub id: i64,
    pub text: String,
    pub completed: bool,
    pub created_at: i64,
    pub updated_at: i64,
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
    
    pub fn insert(&self, req: &CreateTaskRequest) -> SqliteResult<i64> {
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

    pub fn get_all(&self) -> SqliteResult<Vec<Task>> {
        let conn = self.0.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, text, completed, created_at, updated_at FROM tasks ORDER BY created_at DESC")?;
        
        let task_iter = stmt.query_map([], |row: &Row| {
            Ok(Task {
                id: row.get(0)?,
                text: row.get(1)?,
                completed: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })?;
        
        let mut tasks = Vec::new();
        for task in task_iter {
            tasks.push(task?);
        }
        
        Ok(tasks)
    }
}