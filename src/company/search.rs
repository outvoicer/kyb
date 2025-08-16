use crate::company::company::Company;
use rusqlite::{Connection, Result, params};
use std::error::Error;

struct SearchResult {
    name: String,
    reg_code: String,
    address: Option<String>,
    zip: Option<u32>,
    legal_form: String,
    // closed: Option<String>,
}
pub async fn search_by_name(
    conn: &Connection,
    name: &String,
) -> Result<Vec<Company>, Box<dyn Error>> {
    let mut stmt = conn.prepare(
        "SELECT name, reg_code, address, zip, legal_form FROM company WHERE LOWER(name) LIKE LOWER(?1 || '%') LIMIT 10"
    )?;

    // Execute the query and collect the results
    let query = stmt.query_map(params![name], |row| {
        Ok(SearchResult {
            name: row.get(0)?,
            reg_code: row.get(1)?,
            address: row.get(2)?,
            zip: row.get(3)?,
            legal_form: row.get(4)?,
        })
    })?;

    let mut search_results: Vec<Company> = Vec::new();

    for result in query {
        let search_result = result?;
        let company = Company {
            name: search_result.name,
            reg_code: search_result.reg_code,
            address: search_result.address,
            zip: search_result.zip,
            legal_form: search_result.legal_form,
        };
        search_results.push(company);
    }

    Ok(search_results)
}
