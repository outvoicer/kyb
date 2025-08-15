use crate::company::input_company::InputCompany;
use crate::db::create_table::create_table;
use csv::Reader;
use rusqlite::{Connection, Result, params};
use std::error::Error;
use std::io::Cursor;

pub async fn import_companies_from_csv(
    conn: &mut Connection,
    mut rdr: Reader<Cursor<String>>,
) -> Result<(), Box<dyn Error>> {
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

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::company::import::import_companies_from_csv;
    use crate::db::create_table::create_table;
    use actix_web::test;
    use rusqlite::Connection;
    use std::fs::File;
    use std::io::{Cursor, Read};

    #[test]
    async fn company_import_from_sample() {
        let mut conn = Connection::open_in_memory().unwrap();
        let _ = create_table(&conn).await;
        let path = "./src/company/company.csv";
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents);
        let cursor = Cursor::new(contents);

        let rdr = csv::ReaderBuilder::new()
            .delimiter(b';')
            .from_reader(cursor);

        match import_companies_from_csv(&mut conn, rdr).await {
            Ok(_) => {
                assert!(true);
            }
            Err(err) => {
                println!("{}", err);
                assert!(false)
            }
        };

        //import_companies_from_csv(&conn)
    }
}
