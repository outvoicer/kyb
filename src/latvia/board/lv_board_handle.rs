use crate::error::KybError;
use crate::latvia::board::log::log;
use crate::latvia::board::query::Query;
use crate::latvia::board::verify::validate_and_verify::validate_and_verify;
use actix_web::web;
use rusqlite::Connection;

pub async fn lv_board_handle(db: &Connection, query: web::Json<Query>) -> Result<i64, KybError> {
    //let db = get_db()?;
    match validate_and_verify(&db, &query).await {
        Ok(_) => {
            // LOG
            let new_id = log(&db, &query, true, None)?;
            Ok(new_id)
        }
        Err(err) => match err {
            // VERIFICATION ERROR
            KybError::StringError(err) => {
                // LOG
                let error_id = log(&db, &query, false, Some(&err.to_string()))?;
                let error = format!("{}. Error id: {}", err, error_id,);
                Err(KybError::StringError(error))
            }
            // SERVER ERROR
            _ => {
                // LOG
                let error_id = log(&db, &query, false, Some(&err.to_string()))?;
                let error = format!("Server down. Error id: {}", error_id);
                Err(KybError::StringError(error))
            }
        },
    }
}
