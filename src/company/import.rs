use crate::company::company::Company;
use crate::company::input_company::InputCompany;
use crate::company::notmalize::normalize_string;
use crate::company::parse_address::parse_address;
use csv::Reader;
use rusqlite::{Connection, Result, params};
use std::error::Error;
use std::io::Cursor;

pub async fn import_companies_from_csv(
    conn: &mut Connection,
    mut rdr: Reader<Cursor<String>>,
) -> Result<(), Box<dyn Error>> {
    // CREATE TABLE, IF DOES NOT EXIST
    Company::create_table(&conn).await?;
    // DELETE ALL EXISTING RECORDS
    conn.execute("DELETE FROM company", [])?;

    // Begin a transaction
    let transaction = conn.transaction()?;

    {
        let mut stmt = transaction.prepare(
                "INSERT INTO company (reg_code, name, normal_name, city, address, zip, legal_form) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            )?;
        for result in rdr.deserialize() {
            let input_company: InputCompany = result?;
            // ADD COMPANY IF COMPANY IS NOT CLOSED
            let company_is_open = input_company.closed != "L".to_string();
            if company_is_open {
                let (name, normal_name) =
                    get_name_and_normal_name(input_company.name_in_quotes, input_company.name);
                let (city, address) = get_city_and_address(input_company.address);
                stmt.execute(params![
                    input_company.regcode,
                    name,
                    normal_name,
                    city,
                    address,
                    input_company.index,
                    input_company.r#type,
                ])?;
            }
        }
    }

    transaction.commit()?;

    Ok(())
}

pub fn get_city_and_address(input_address: Option<String>) -> (Option<String>, Option<String>) {
    let mut city = None;
    let mut address = None;
    match input_address {
        Some(input_address) => {
            let (c, a) = parse_address(&input_address);
            city = Some(c);
            address = Some(a);
        }
        None => {}
    }
    (city, address)
}

pub fn get_name_and_normal_name(name_1: String, name_2: String) -> (String, String) {
    let mut name = name_1;
    if name == "" {
        name = name_2;
    }
    let normal_name = normalize_string(&name);
    (name, normal_name)
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

        let conn = create_test_db().await.unwrap();
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
