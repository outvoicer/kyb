use crate::error::KybError;
use crate::latvia::board::query::Query;
use rusqlite::{Connection, params};

pub async fn is_board_member(conn: &Connection, query: &Query) -> Result<(), KybError> {
    let mut stmt =
        conn.prepare("SELECT personal_code FROM officers WHERE name = ?1 AND reg_code = ?2")?;

    let trimmed_name = query.name.trim();

    let mut rows = stmt.query(params![trimmed_name, query.reg_code])?;

    let personal_code_beginning = &query.personal_code[0..6];

    while let Some(row) = rows.next()? {
        let db_personal_code: String = row.get(0)?;
        if db_personal_code.len() == 0 {
            // IF NO PERSONAL CODE IN DB - ALLOW
            return Ok(());
        } else {
            // IF PERSONAL CODE EXISTS IN DB - VERIFY
            if db_personal_code.starts_with(personal_code_beginning) {
                return Ok(());
            }
        }
    }

    Err(KybError::StringError("Not member of board".to_string()))
}
