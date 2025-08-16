use crate::company::company::Company;
use rusqlite::Connection;
use std::error::Error;

impl Company {
    pub async fn create_table(conn: &Connection) -> Result<(), Box<dyn Error>> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS company (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                reg_code TEXT NOT NULL,
                name TEXT NOT NULL,
                normal_name TEXT NOT NULL,
                city TEXT,
                address TEXT,
                zip INTEGER,
                legal_form TEXT
            )",
            [],
        )?;
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_normal_name ON company (normal_name)",
            [],
        )?;

        Ok(())
    }
}
