use crate::config::KybConfig;
use crate::public_institution::import::import_public_institutions_from_csv;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use reqwest::get;
use rusqlite::Result;
use std::error::Error;
use std::io::Cursor;

pub async fn fetch_new_public_institution_data(
    conn: &mut PooledConnection<SqliteConnectionManager>,
) -> Result<(), Box<dyn Error>> {
    let url = KybConfig::SOURCE_PUBLIC_INSTITUTIONS;
    println!("getting {}", url);
    let response = get(url).await?.text().await?;
    let cursor = Cursor::new(response);

    let rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(cursor);

    import_public_institutions_from_csv(conn, rdr).await?;
    Ok(())
}
