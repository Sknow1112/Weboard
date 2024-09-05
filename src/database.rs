use rusqlite::{Connection, Result};
use crate::whiteboard::DrawAction;
use std::sync::Mutex;

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn new(conn: Connection) -> Self {
        Database { conn: Mutex::new(conn) }
    }

    pub fn save_action(&self, action: &DrawAction) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO actions (x, y, color, is_eraser) VALUES (?1, ?2, ?3, ?4)",
            (action.x, action.y, &action.color, action.is_eraser),
        )?;
        Ok(())
    }

    pub fn get_current_state(&self) -> Result<Vec<DrawAction>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT x, y, color, is_eraser FROM actions ORDER BY id")?;
        let actions = stmt.query_map([], |row| {
            Ok(DrawAction {
                x: row.get(0)?,
                y: row.get(1)?,
                color: row.get(2)?,
                is_eraser: row.get(3)?,
            })
        })?;
        actions.collect()
    }

    pub fn clear_whiteboard(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM actions", [])?;
        Ok(())
    }
}

pub fn init_db() -> Result<Database> {
    let conn = Connection::open("whiteboard.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS actions (
            id INTEGER PRIMARY KEY,
            x REAL NOT NULL,
            y REAL NOT NULL,
            color TEXT NOT NULL,
            is_eraser BOOLEAN NOT NULL
        )",
        [],
    )?;
    Ok(Database::new(conn))
}
