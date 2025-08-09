use crate::{db::query::Query, error::KybError};
use chrono::Local;
use rusqlite::{Connection, params};

pub fn log(
    conn: &Connection,
    query: &Query,
    decition: bool,
    error: Option<&String>,
) -> Result<i64, KybError> {
    let now = Local::now();
    let formatted_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
    let err = match error {
        Some(err) => err,
        None => "",
    };
    conn.execute(
        "INSERT INTO log (decition, name, personal_code, reg_code, error, time) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![decition, &query.name, &query.personal_code, query.reg_code, err, formatted_time],
    )?;
    let last_id = conn.last_insert_rowid();
    Ok(last_id)
}
