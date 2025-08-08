use crate::error::KybError;
use rusqlite::{Connection, Result};

pub async fn create_table(conn: &Connection) -> Result<(), KybError> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS officers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            reg_code TEXT NOT NULL,
            name TEXT NOT NULL,
            personal_code TEXT NOT NULL,
            position TEXT
        )",
        [],
    )
    .unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS log (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            reg_code TEXT NOT NULL,
            name TEXT NOT NULL,
            personal_code TEXT NOT NULL,
            decition INTEGER NOT NULL CHECK (decition IN (0, 1)),
            time TEXT NOT NULL
        )",
        [],
    )
    .unwrap();

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_officers_name_reg_code ON officers (name, reg_code)",
        [],
    )?;

    Ok(())
}
