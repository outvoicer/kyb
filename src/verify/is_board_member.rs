use crate::db::query::Query;
use rusqlite::{Connection, params};

pub async fn is_board_member(conn: &Connection, query: &Query) -> bool {
    let mut stmt = conn
        .prepare("SELECT personal_code FROM officers WHERE name = ?1 AND reg_code = ?2")
        .unwrap();

    let mut rows = stmt.query(params![query.name, query.reg_code]).unwrap();

    let personal_code_beginning = &query.personal_code[0..6];

    while let Some(row) = rows.next().unwrap() {
        let db_personal_code: String = row.get(0).unwrap();
        if db_personal_code.len() == 0 {
            // IF NO PERSONAL CODE IN DB - ALLOW
            return true;
        } else {
            // IF PERSONAL CODE EXISTS IN DB - VERIFY
            if db_personal_code.starts_with(personal_code_beginning) {
                return true;
            }
        }
    }

    false
}
