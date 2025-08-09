use crate::db::query::Query;
use crate::db::{get_db::get_db, log::log};
use crate::error::KybError;
use crate::verify::validate_and_verify::validate_and_verify;
use actix_web::web;

pub async fn handle_lv(query: web::Json<Query>) -> Result<i64, KybError> {
    let db = get_db()?;
    match validate_and_verify(&db, &query).await {
        Ok(_) => {
            // ERROR HANDLE SEE UNWRAP - ANNAB EMPTY REPLY KUI DB-D EI OLE
            let new_id = log(&db, &query, true, None)?;
            Ok(new_id)
        }
        Err(err) => match err {
            // VERIFICATION ERROR
            KybError::StringError(err) => {
                let error_id = log(&db, &query, false, Some(&err.to_string()))?;
                let error = format!("{} {}", error_id, err);
                Err(KybError::StringError(error))
            }
            _ => {
                // SERVER ERROR
                eprintln!("server down: {:?}", &err);
                let error_id = log(&db, &query, false, Some(&err.to_string()))?;
                let error = format!("Server down. Error id: {}", error_id);
                Err(KybError::StringError(error))
            }
        },
    }
}
