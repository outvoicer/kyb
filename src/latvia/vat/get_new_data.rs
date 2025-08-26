use crate::config::KybConfig;
use crate::latvia::vat::import::get_vat_data;
use csv::ReaderBuilder;
use reqwest::get;
use rusqlite::Result;
use std::collections::HashSet;
use std::error::Error;
use std::io::Cursor;

pub async fn fetch_new_vat_data() -> Result<HashSet<String>, Box<dyn Error>> {
    let url = KybConfig::SOURCE_VAT;
    println!("getting {}", url);
    let response = get(url).await?.text().await?;
    let cursor = Cursor::new(response);
    let rdr = ReaderBuilder::new()
        .has_headers(true) // The first row is a header
        .trim(csv::Trim::All) // Trim whitespace around fields
        .from_reader(cursor);

    let vat_table = get_vat_data(rdr).await?;
    Ok(vat_table)
}
