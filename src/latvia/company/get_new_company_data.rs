use crate::config::KybConfig;
use crate::latvia::company::import::import_companies_from_csv;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use reqwest::get;
use rusqlite::Result;
use std::collections::HashSet;
use std::error::Error;
use std::io::Cursor;

pub async fn fetch_new_company_data(
    conn: &mut PooledConnection<SqliteConnectionManager>,
    vat_table: &HashSet<String>,
) -> Result<(), Box<dyn Error>> {
    let url = KybConfig::SOURCE_COMPANIES;
    println!("getting {}", url);
    let response = get(url).await?.text().await?;
    let cursor = Cursor::new(response);

    let rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(cursor);

    import_companies_from_csv(conn, rdr, &vat_table).await?;
    Ok(())
}
