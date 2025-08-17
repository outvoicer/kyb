use rusqlite::Connection;
use std::error::Error;

pub async fn create_company_log_table(conn: &Connection) -> Result<(), Box<dyn Error>> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS company_log (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                input TEXT,
                search_name TEXT,
                results TEXT,
                error TEXT
            )",
        [],
    )?;

    Ok(())
}
