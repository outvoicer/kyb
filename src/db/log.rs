use crate::db::query::Query;
use chrono::Local;
use rusqlite::{Connection, params};
use std::error::Error;

pub fn log(conn: &Connection, query: &Query, decition: bool) -> Result<i64, Box<dyn Error>> {
    let now = Local::now();
    conn.execute(
        "INSERT INTO log (name, personal_code, reg_code, decition, time) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![&query.name, &query.personal_code, query.reg_code, decition, now.to_string()],
    )?;
    let last_id = conn.last_insert_rowid();
    Ok(last_id)
}
