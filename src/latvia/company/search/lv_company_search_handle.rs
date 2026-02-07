use crate::error::KybError;
use crate::latvia::company::company::Company;
use crate::latvia::company::log::log::log_search;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CompanySearchQuery {
    pub name: String,
}

pub async fn lv_company_search_handle(
    db: &Connection,
    query: CompanySearchQuery,
) -> Result<Vec<Company>, KybError> {
    match Company::search_by_name(db, &query.name).await {
        Ok(results) => return Ok(results),
        Err(err) => {
            log_search(&db, &query.name, &"".to_string(), &vec![], err.to_string()).await;
            eprintln!("{}", err);
            let empty = vec![];
            return Ok(empty);
        }
    };
}
