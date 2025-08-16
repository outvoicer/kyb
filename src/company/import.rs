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
            let input_company: InputCompany = result?;
            stmt.execute(params![
                input_company.regcode,
                input_company.name_in_quotes,
                input_company.address,
                input_company.index,
                input_company.regtype_text,
                input_company.closed,
            ])?;
        }
    }

    transaction.commit()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::company::company::Company;
    use crate::company::import::import_companies_from_csv;
    use crate::company::search::search_by_name;
    use crate::db::create_table::create_table;
    use actix_web::test;
    use rusqlite::Connection;
    use std::error::Error;
    use std::fs::File;
    use std::io;
    use std::io::{Cursor, Read};

    async fn get_first_result(
        conn: &Connection,
        search_term: &String,
    ) -> Result<Company, Box<dyn Error>> {
        match search_by_name(&conn, &search_term).await {
            Ok(companies) => {
                if let Some(first_company) = companies.first() {
                    return Ok(first_company.clone());
                } else {
                    let err: Box<dyn Error> = Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "No company found",
                    ));
                    return Err(err);
                }
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    #[test]
    async fn company_import_from_sample() {
        let search_term_1 = "KRASTNIEKI".to_string();
        let search_term_2 = "VALKRĪG".to_string();

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
                match get_first_result(&conn, &search_term_1).await {
                    Ok(first_result) => {
                        let reg_code: String = "41202013815".to_string();
                        assert_eq!(
                            first_result.reg_code, reg_code,
                            "The registration code does not match the expected value."
                        );
                    }
                    Err(err) => {
                        eprintln!("{}", err);
                        assert!(false, "Search did not respond correctly")
                    }
                }
                // DO NOT GET "VALKRĪG" (44102037886) AS IT'S DELETED
                match get_first_result(&conn, &search_term_2).await {
                    Ok(_) => {
                        assert!(false, "First result should not appear");
                    }
                    Err(err) => {
                        if let Some(err) = err.downcast_ref::<io::Error>() {
                            assert_eq!(err.kind(), io::ErrorKind::Other);
                            assert_eq!(err.to_string(), "No company found");
                        } else {
                            panic!("Expected an io::Error");
                        }
                    }
                }
            }
            Err(err) => {
                eprintln!("{}", err);
                assert!(false, "company importing error")
            }
        };
    }
}
