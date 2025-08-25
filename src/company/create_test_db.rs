use crate::company::company::Company;
use crate::company::import::import_companies_from_csv;
use crate::db::get_db::Pool;
use crate::vat::read_sample_data::read_sample_vat_data;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use std::error::Error;
use std::fs::File;
use std::io::{Cursor, Read};

#[allow(dead_code)]
pub async fn create_test_db() -> Result<Pool, Box<dyn Error>> {
    let manager = SqliteConnectionManager::memory();
    let pool = Pool::new(manager).unwrap();
    let mut conn: PooledConnection<SqliteConnectionManager> =
        pool.get().expect("Couldn't get db connection from pool");

    // let mut conn = Connection::open_in_memory()?;
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
    let vat_table = read_sample_vat_data().await?;
    import_companies_from_csv(&mut conn, rdr, vat_table).await?;
    Ok(pool)
}
