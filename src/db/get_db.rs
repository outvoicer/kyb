use crate::db::db_file::db_file;
use crate::error::KybError;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Result;

pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;

pub fn get_db() -> Result<Pool, KybError> {
    let file = db_file()?;
    let manager = SqliteConnectionManager::file(file);
    let pool = Pool::new(manager).unwrap();
    Ok(pool)
}
