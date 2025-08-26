use crate::latvia::company::company::Company;
use crate::latvia::company::notmalize::normalize_string;
use crate::latvia::government::parse_address::parse_gov_address;
use crate::latvia::government::public_institution::PublicInstitution;
use crate::latvia::vat::vat_status::vat_status;
use csv::Reader;
use rusqlite::{Connection, Result, params};
use std::collections::HashSet;
use std::error::Error;
use std::io::Cursor;

pub async fn import_public_institutions_from_csv(
    conn: &mut Connection,
    mut rdr: Reader<Cursor<String>>,
    vat_table: &HashSet<String>,
) -> Result<(), Box<dyn Error>> {
    // CREATE TABLE, IF DOES NOT EXIST
    Company::create_table(&conn).await?;
    // DELETE ALL EXISTING RECORDS
    conn.execute("DELETE FROM company WHERE public_sector = '1'", [])?;

    // Begin a transaction
    let transaction = conn.transaction()?;

    {
        let mut stmt = transaction.prepare(
                "INSERT INTO company (legal_form, name, city, address, zip, normal_name, public_sector, reg_code, vat, vat_number) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            )?;
        for result in rdr.deserialize() {
            let input: PublicInstitution = result?;
            // ADD COMPANY IF COMPANY IS NOT CLOSED
            let still_exists = input.Status == "REGISTERED".to_string();
            if still_exists {
                let normal_name = normalize_string(&input.name);
                let (city, address, zip) = parse_gov_address(&input.address);
                let (vat, vat_number) = vat_status(&input.registrationNumber, &vat_table);
                stmt.execute(params![
                    "", // EMPTY STRING FOR GOV INSTITUTIONS
                    input.name,
                    city,
                    address,
                    zip,
                    normal_name,
                    "1".to_string(),
                    input.registrationNumber,
                    vat,
                    vat_number
                ])?;
            }
        }
    }

    transaction.commit()?;

    Ok(())
}
