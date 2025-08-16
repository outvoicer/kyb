use crate::company::company::Company;
use crate::company::import::import_companies_from_csv;
use rusqlite::Connection;
use std::error::Error;
use std::fs::File;
use std::io::{Cursor, Read};

pub async fn create_test_db() -> Result<Connection, Box<dyn Error>> {
    let mut conn = Connection::open_in_memory()?;
    // ADD TABLE SCHEMA
    Company::create_table(&conn).await?;
    // GET SAMPLE DATA
    let path = "./src/company/company.csv";
    let mut file = File::open(path)?;
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents)?;
    let cursor = Cursor::new(contents);

    let rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(cursor);
    // SAVE DATA
    let _ = import_companies_from_csv(&mut conn, rdr).await?;
    Ok(conn)
}
