use crate::company::company::Company;
use rusqlite::{Result, Rows};
use std::error::Error;

struct SearchResult {
    name: String,
    reg_code: String,
    address: Option<String>,
    city: Option<String>,
    zip: Option<u32>,
    legal_form: String,
}

pub async fn search_map_results(mut rows: Rows<'_>) -> Result<Vec<Company>, Box<dyn Error>> {
    //let mut search_results: Vec<SearchResult> = Vec::new();
    let mut results = Vec::new();

    while let Some(row) = rows.next()? {
        let result = SearchResult {
            name: row.get(0)?,
            reg_code: row.get(1)?,
            city: row.get(2)?,
            address: row.get(3)?,
            zip: row.get(4)?,
            legal_form: row.get(5)?,
        };
        results.push(result);
    }

    let mut search_results: Vec<Company> = Vec::new();

    for search_result in results {
        // let search_result = result?;
        let company = Company {
            name: search_result.name,
            reg_code: search_result.reg_code,
            city: search_result.city,
            address: search_result.address,
            zip: search_result.zip,
            legal_form: search_result.legal_form,
        };
        search_results.push(company);
    }

    Ok(search_results)
}
