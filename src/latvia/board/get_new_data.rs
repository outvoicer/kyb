use crate::config::KybConfig;
use crate::latvia::board::Officer;
use crate::latvia::board::create_table::create_table;
use chrono::Local;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use reqwest::get;
use rusqlite::{Result, params};
use std::error::Error;
use std::io::Cursor;

fn print(text: &str) {
    let now = Local::now();
    println!("{} {}", now, text);
}

pub async fn fetch_and_store_data(
    conn: &mut PooledConnection<SqliteConnectionManager>,
) -> Result<(), Box<dyn Error>> {
    print("Get new data");
    let url = KybConfig::SOURCE_MEMBER_OF_BOARD;
    let response = get(url).await?.text().await?;
    let cursor = Cursor::new(response);

    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(cursor);

    print("Store data");
    // CREATE TABLE, IF DOES NOT EXIST
    create_table(&conn).await?;
    // DELETE ALL EXISTING RECORDS
    conn.execute("DELETE FROM officers", [])?;
    // Begin a transaction
    let transaction = conn.transaction()?;

    {
        let mut stmt = transaction.prepare(
            "INSERT INTO officers (reg_code, name, personal_code, position) VALUES (?1, ?2, ?3, ?4)",
        )?;
        for result in rdr.deserialize() {
            let officer: Officer = result?;
            stmt.execute(params![
                officer.at_legal_entity_registration_number,
                officer.name,
                officer.latvian_identity_number_masked,
                officer.position
            ])?;
        }
    }

    transaction.commit()?;

    print("Data saved");
    Ok(())
}
