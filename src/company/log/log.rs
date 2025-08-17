use rusqlite::{Connection, params};

use crate::company::company::Company;

pub async fn log_search(
    conn: &Connection,
    input: &String,
    search_name: &String,
    results: &Vec<Company>,
) {
    let mut names: Vec<String> = Vec::new();
    for result in results {
        names.push(result.name.clone())
    }
    let found_names = names.join(", ");
    let insert = conn.execute(
        "INSERT INTO company_log (input, search_name, results) VALUES (?1, ?2, ?3)",
        params![input, search_name, found_names],
    );
    if insert.is_err() {
        eprintln!("search log error: {:?}", insert);
    }
    // let last_id = conn.last_insert_rowid();
    // Ok(last_id)
    ()
}
