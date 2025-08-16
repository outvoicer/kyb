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
                "INSERT INTO company (reg_code, name, address, zip, legal_form) VALUES (?1, ?2, ?3, ?4, ?5)",
            )?;
        for result in rdr.deserialize() {
            let input_company: InputCompany = result?;
            if input_company.closed != "L".to_string() {
                stmt.execute(params![
                    input_company.regcode,
                    input_company.name_in_quotes,
                    input_company.address,
                    input_company.index,
                    input_company.regtype_text,
                ])?;
            }
        }
    }

    transaction.commit()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::company::create_test_db::create_test_db;
    use crate::company::get_first_result::get_first_result;
    use actix_web::test;

    #[test]
    async fn company_import_from_sample() {
        let search_term_1 = "House of Glory".to_string();
        let reg_code_1 = "40008234596".to_string();
        let search_term_2 = "VALKRĪG".to_string();

        let conn = create_test_db().await.unwrap();
        // THIS SEARCH SHOULD FIND
        let first_result = get_first_result(&conn, &search_term_1).await.unwrap();
        assert_eq!(
            first_result.reg_code, reg_code_1,
            "The registration code does not match the expected value."
        );
        // THIS SEARCH SHOULD NOT FIND
        // DO NOT GET "VALKRĪG" (44102037886) AS IT'S DELETED
        match get_first_result(&conn, &search_term_2).await {
            Ok(_) => {
                assert!(false, "First result should not appear");
            }
            Err(err) => {
                if let Some(err) = err.downcast_ref::<std::io::Error>() {
                    assert_eq!(err.kind(), std::io::ErrorKind::Other);
                    assert_eq!(err.to_string(), "No company found");
                } else {
                    panic!("Expected company not found");
                }
            }
        }
    }
}
