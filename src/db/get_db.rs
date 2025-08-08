use crate::db::db_file::db_file;
use crate::error::AppError;
use rusqlite::{Connection, Result};

pub fn get_db() -> Result<Connection, AppError> {
    let file = db_file()?;
    let conn = Connection::open(file)?;
    Ok(conn)
}
