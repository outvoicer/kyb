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
    let err = match error {
        Some(err) => err,
        None => "",
    };
    conn.execute(
        "INSERT INTO log (name, personal_code, reg_code, decition, error, time) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![&query.name, &query.personal_code, query.reg_code, decition, err, now.to_string()],
    )?;
    let last_id = conn.last_insert_rowid();
    Ok(last_id)
}
