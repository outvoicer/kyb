//use super::init::initialize_db;
use crate::db::create_table::create_table;
use crate::db::db_file::db_file;
use crate::error::AppError;
//use rusqlite::Connection;
use rusqlite::{Connection, Result};

pub async fn create_db(password: &String) -> Result<Connection, AppError> {
    let file = db_file()?;
    let conn = Connection::open(file)?;
    // Set the encryption key
    let pwd = password.trim();
    //conn.pragma_update(None, "key", pwd)?;
    //initialize_db(&conn, &pwd.to_string())?;
    create_table(&conn).await?;
    Ok(conn)
}
