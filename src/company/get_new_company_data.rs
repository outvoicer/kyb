use crate::company::import::import_companies_from_csv;
use crate::config::KybConfig;
use crate::db::get_db::get_db;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use reqwest::get;
use rusqlite::Result;
use std::error::Error;
use std::io::Cursor;

pub async fn fetch_new_company_data() -> Result<(), Box<dyn Error>> {
    let url = KybConfig::SOURCE_COMPANIES;
    println!("getting {}", url);
    let response = get(url).await?.text().await?;
    let cursor = Cursor::new(response);

    let rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(cursor);

    let pool = get_db()?;
    let mut conn: PooledConnection<SqliteConnectionManager> =
        pool.get().expect("Couldn't get db connection from pool");

    import_companies_from_csv(&mut conn, rdr).await?;
    Ok(())
}
