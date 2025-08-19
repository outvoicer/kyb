use crate::company::company::Company;
use crate::company::notmalize::normalize_string;
use crate::public_institution::parse_address::parse_gov_address;
use crate::public_institution::public_institution::PublicInstitution;
use csv::Reader;
use rusqlite::{Connection, Result, params};
use std::error::Error;
use std::io::Cursor;

pub async fn import_public_institutions_from_csv(
    conn: &mut Connection,
    mut rdr: Reader<Cursor<String>>,
) -> Result<(), Box<dyn Error>> {
    // CREATE TABLE, IF DOES NOT EXIST
    Company::create_table(&conn).await?;
    // DELETE ALL EXISTING RECORDS
    conn.execute("DELETE FROM company WHERE eadrese = TRUE", [])?;

    // Begin a transaction
    let transaction = conn.transaction()?;

    {
        let mut stmt = transaction.prepare(
                "INSERT INTO company (legal_form, name, city, address, zip, normal_name, reg_code) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            )?;
        for result in rdr.deserialize() {
            let input: PublicInstitution = result?;
            // ADD COMPANY IF COMPANY IS NOT CLOSED
            let still_exists = input.status != "REGISTERED".to_string();
            if still_exists {
                let normal_name = normalize_string(&input.status);
                let (city, address, zip) = parse_gov_address(&input.address);
                stmt.execute(params![
                    "", // LIBRARY IS NOT SIA
                    input.name,
                    city,
                    address,
                    zip,
                    normal_name,
                    input.registration_number,
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
    use r2d2::PooledConnection;
    use r2d2_sqlite::SqliteConnectionManager;

    #[test]
    async fn company_import_from_sample() {
        let search_term_1 = "House of Glory".to_string();
        let reg_code_1 = "40008234596".to_string();

        let pool = create_test_db().await.unwrap();
        let conn: PooledConnection<SqliteConnectionManager> =
            pool.get().expect("Couldn't get db connection from pool");

        // THIS SEARCH SHOULD FIND
        let first_result = get_first_result(&conn, &search_term_1).await.unwrap();
        assert_eq!(
            first_result.reg_code, reg_code_1,
            "The registration code does not match the expected value."
        );
        // THIS SEARCH SHOULD NOT FIND
        // DO NOT GET "VALKRĪG" (44102037886) AS IT'S DELETED
        let search_term_2 = "VALKRĪG".to_string();
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
