use crate::latvia::company::company::Company;
use chrono::Local;
use rusqlite::{Connection, params};

pub async fn log_search(
    conn: &Connection,
    input: &String,
    search_name: &String,
    results: &Vec<Company>,
    error: String,
) {
    let mut names: Vec<String> = Vec::new();
    for result in results {
        names.push(result.name.clone())
    }
    let found_names = names.join(", ");
    // Time
    let now = Local::now();
    let formatted_time = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let insert = conn.execute(
        "INSERT INTO company_log (input, search_name, results, time, error) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![input, search_name, found_names, formatted_time, error],
    );
    if insert.is_err() {
        eprintln!("search log error: {:?}", insert);
    }
    ()
}
