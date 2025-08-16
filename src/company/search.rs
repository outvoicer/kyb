use crate::company::company::Company;
use crate::company::notmalize::normalize_string;
use crate::company::search_map_results::search_map_results;
use rusqlite::{Connection, Result, params};
use std::error::Error;

pub async fn search_by_name(
    conn: &Connection,
    name: &String,
) -> Result<Vec<Company>, Box<dyn Error>> {
    let normalized_name = normalize_string(name);

    let mut stmt = conn.prepare(
        "SELECT name, reg_code, address, zip, legal_form FROM company WHERE normal_name LIKE ('%' || ?1 || '%') LIMIT 10"
    )?;

    let rows = stmt.query(params![normalized_name])?;
    let search_results = search_map_results(rows).await?;
    Ok(search_results)
}

#[cfg(test)]
mod tests {
    use crate::company::create_test_db::create_test_db;
    use crate::company::get_first_result::get_first_result;
    use actix_web::test;

    #[test]
    async fn name_search_lv_letters() {
        let conn = create_test_db().await.unwrap();
        let reg_code = "90000519196".to_string();
        // Raimond fantastic
        let search_term_minus_1 = "Raimond fantastic".to_string();
        let result = get_first_result(&conn, &search_term_minus_1).await.unwrap();
        assert_eq!(
            result.reg_code,
            "40203572370".to_string(),
            "Wrong seartch result."
        );
        // ROMAS KATOĻU BAZNĪCAS RĒZEKNES-AGLONAS DIECĒZE
        let search_term_0 = "ROMAS KATOĻU BAZNĪCAS RĒZEKNES-AGLONAS DIECĒZE".to_string();
        let result = get_first_result(&conn, &search_term_0).await.unwrap();
        assert_eq!(result.reg_code, reg_code, "Wrong search result.");
        // KATOĻU
        let search_term_1 = "KATOĻU".to_string();
        let result = get_first_result(&conn, &search_term_1).await.unwrap();
        assert_eq!(result.reg_code, reg_code, "Wrong search result.");
        // katolu
        let search_term_2 = "katolu".to_string();
        let result = get_first_result(&conn, &search_term_2).await.unwrap();
        assert_eq!(result.reg_code, reg_code, "Wrong search result.");
        // katoļu
        let search_term_3 = "katoļu".to_string();
        let result = get_first_result(&conn, &search_term_3).await.unwrap();
        assert_eq!(result.reg_code, reg_code, "Wrong search result.");
        // KATOĻU BAZNĪCAS RĒZEKNES
        let search_term_4 = "KATOĻU BAZNĪCAS RĒZEKNES".to_string();
        let result = get_first_result(&conn, &search_term_4).await.unwrap();
        assert_eq!(result.reg_code, reg_code, "Wrong seartch result.");
        // KATOĻU
        let search_term_5 = "KATOĻU".to_string();
        let result = get_first_result(&conn, &search_term_5).await.unwrap();
        assert_eq!(result.reg_code, reg_code, "Wrong seartch result.");
        // DIECĒZE
        let search_term_6 = "DIECĒZE".to_string();
        let result = get_first_result(&conn, &search_term_6).await.unwrap();
        assert_eq!(result.reg_code, reg_code, "Wrong seartch result.");
    }
}
