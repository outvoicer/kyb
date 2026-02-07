use crate::latvia::company::company::Company;
//use crate::company::search::search_by_name;
use rusqlite::Connection;
use std::error::Error;

#[allow(dead_code)]
pub async fn get_first_result(
    conn: &Connection,
    search_term: &String,
) -> Result<Company, Box<dyn Error>> {
    match Company::search_by_name(&conn, &search_term, false).await {
        Ok(companies) => {
            // println!("{:?}", &companies);
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
