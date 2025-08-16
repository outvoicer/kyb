use crate::company::company::Company;
use crate::company::search_map_results::search_map_results;
use rusqlite::{Connection, Result, params};
use std::error::Error;

pub async fn search_by_name(
    conn: &Connection,
    name: &String,
) -> Result<Vec<Company>, Box<dyn Error>> {
    let mut stmt = conn.prepare(
        "SELECT name, reg_code, address, zip, legal_form FROM company WHERE LOWER(name) LIKE LOWER(?1 || '%') LIMIT 10"
    )?;

    let rows = stmt.query(params![name])?;
    let search_results = search_map_results(rows).await?;
    Ok(search_results)
}
