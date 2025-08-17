use crate::company::company::Company;
use crate::db::get_db::get_db;
use crate::error::KybError;
use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CompanySearchQuery {
    pub name: String,
}

pub async fn lv_company_search_handle(
    query: web::Json<CompanySearchQuery>,
) -> Result<Vec<Company>, KybError> {
    let db = get_db()?;

    match Company::search_by_name(&db, &query.name).await {
        Ok(results) => return Ok(results),
        Err(err) => {
            eprintln!("{}", err);
            let v = vec![];
            return Ok(v);
        }
    };
}
