use crate::config::KybConfig;
use crate::vat::import::get_VAT_data;
use csv::ReaderBuilder;
use reqwest::get;
use rusqlite::Result;
use std::error::Error;
use std::io::Cursor;

pub async fn fetch_new_VAT_data() -> Result<(), Box<dyn Error>> {
    let url = KybConfig::SOURCE_VAT;
    println!("getting {}", url);
    let response = get(url).await?.text().await?;
    let cursor = Cursor::new(response);
    let mut rdr = ReaderBuilder::new()
        .has_headers(true) // The first row is a header
        .trim(csv::Trim::All) // Trim whitespace around fields
        .from_reader(cursor);

    get_VAT_data(rdr).await?;
    Ok(())
}
