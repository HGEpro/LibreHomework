use rusqlite::{Connection, Result};
use dirs_next::config_dir;

#[derive(Debug)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub subject: String,
    pub description: String,
    pub expires_at: u64
}
pub struct Database {
    path: String
}
impl Database {
    pub fn new() -> Option<Self> {
        let path = config_dir().unwrap().join("LibreHomework/LibreHomework.db");

        if !path.exists() {
            return None;
        } else {
            return Some(Database { path: path.to_str().unwrap().to_string() });
        }
    }

    fn init_connection(&self) -> Result<Connection> {
        Connection::open(self.path.clone())
    }

    pub fn check(&mut self) -> Result<Vec<Task>, rusqlite::Error> {
        let mut tasks: Vec<Task> = Vec::new();
        let conn = self.init_connection()?;
        let mut stmt = conn.prepare("SELECT * FROM tasks")?;
        let mut rows = stmt.query([])?;

        while let Some(row) = rows.next()? {

            let id = row.get(0)?;
            let name = row.get(1)?;
            let subject = row.get(2)?;
            let description = row.get(3)?;
            let expires_at = row.get(4)?;

            tasks.push(Task {
                id,
                name,
                subject,
                description,
                expires_at
            });
        }
        Ok(tasks)
    }
}