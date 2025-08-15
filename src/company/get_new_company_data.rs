use crate::company::input_company::InputCompany;
use crate::config::KybConfig;
use crate::db::create_table::create_table;
use crate::db::get_db::get_db;
use chrono::Local;
use reqwest::get;
use rusqlite::{Result, params};
use std::error::Error;
use std::io::Cursor;

fn print(text: &str) {
    let now = Local::now();
    println!("{} {}", now, text);
}

pub async fn fetch_new_company_data() -> Result<(), Box<dyn Error>> {
    print("Get new compay data");

    let url = KybConfig::SOURCE_COMPANIES;
    let response = get(url).await?.text().await?;
    //
    let cursor = Cursor::new(response);

    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(cursor);

    print("Store COMPANY data");

    let mut conn = get_db()?;
    // CREATE TABLE, IF DOES NOT EXIST
    create_table(&conn).await?;
    // DELETE ALL EXISTING RECORDS
    conn.execute("DELETE FROM company", [])?;
    // Begin a transaction
    let transaction = conn.transaction()?;

    {
        let mut stmt = transaction.prepare(
            "INSERT INTO company (reg_code, name, address, zip, legal_form, closed) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        )?;
        for result in rdr.deserialize() {
            let company: InputCompany = result?;
            stmt.execute(params![
                company.regcode,
                company.name_in_quotes,
                company.address,
                company.index,
                company.regtype_text,
                company.closed,
            ])?;
        }
    }

    transaction.commit()?;

    print("Companies data saved");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::verify::setup_test_db::setup_test_db;
    use actix_web::test;

    #[test]
    async fn test_is_officer_success() {
        let conn = setup_test_db().await;
    }
}
