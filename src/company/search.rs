use crate::company::clean_name::clean_company_name;
use crate::company::company::Company;
use crate::company::log::log::log_search;
use crate::company::notmalize::normalize_string;
use crate::company::search_map_results::search_map_results;
use rusqlite::{Connection, Result, params};
use std::error::Error;

impl Company {
    pub async fn search_by_name(
        conn: &Connection,
        name: &String,
    ) -> Result<Vec<Company>, Box<dyn Error>> {
        let mut stmt = conn.prepare(
            "SELECT legal_form, name, city, address, zip, public_sector, reg_code FROM company WHERE normal_name LIKE ('%' || ?1 || '%') LIMIT 10"
        )?;
        // REMOVE SIA FROM BEGINNING
        let clean_name = clean_company_name(&name);
        // MAKE SMALL CAPS AND LATIN LETTERS
        let normalized_name = normalize_string(&clean_name.to_string());
        if normalized_name == "".to_string() {
            // IF NOTHING LEFT, RETURN EMPTY
            log_search(conn, name, &normalized_name, &vec![], "".to_string()).await;
            return Ok(vec![]);
        }
        // QUERY
        let rows = stmt.query(params![normalized_name])?;
        // MAP RESUTS
        let search_results = search_map_results(rows).await?;
        // LOG RESULTS
        log_search(
            conn,
            name,
            &normalized_name,
            &search_results,
            "".to_string(),
        )
        .await;
        // RETURN
        Ok(search_results)
    }
}

#[cfg(test)]
mod tests {
    use crate::company::create_test_db::create_test_db;
    use crate::company::get_first_result::get_first_result;
    use actix_web::test;
    use r2d2::PooledConnection;
    use r2d2_sqlite::SqliteConnectionManager;

    #[test]
    async fn name_search_lv_letters() {
        let pool = create_test_db().await.unwrap();
        let conn: PooledConnection<SqliteConnectionManager> =
            pool.get().expect("Couldn't get db connection from pool");

        let reg_code = "90000519196".to_string();

        // Raimond fantastic
        let search_term = "Raimond fantastic".to_string();
        let result = get_first_result(&conn, &search_term).await.unwrap();
        assert_eq!(
            result.reg_code,
            "40203572370".to_string(),
            "Wrong seartch result."
        );
        assert_eq!(result.city, Some("Jūrmala".to_string()));
        assert_eq!(result.address, Some("Mellužu prospekts 76".to_string()));
        assert_eq!(result.legal_form, "SIA".to_string());
        // ROMAS KATOĻU BAZNĪCAS RĒZEKNES-AGLONAS DIECĒZE
        let search_term = "ROMAS KATOĻU BAZNĪCAS RĒZEKNES-AGLONAS DIECĒZE".to_string();
        let result = get_first_result(&conn, &search_term).await.unwrap();
        assert_eq!(result.reg_code, reg_code);
        assert_eq!(result.city, Some("Rēzekne".to_string()));
        assert_eq!(result.address, Some("Latgales iela 88".to_string()));
        assert_eq!(result.legal_form, "KAT".to_string());
        // KATOĻU
        let search_term = "KATOĻU".to_string();
        let result = get_first_result(&conn, &search_term).await.unwrap();
        assert_eq!(result.reg_code, reg_code, "Wrong search result.");
        // katolu
        let search_term = "katolu".to_string();
        let result = get_first_result(&conn, &search_term).await.unwrap();
        assert_eq!(result.reg_code, reg_code, "Wrong search result.");
        // katoļu
        let search_term = "katoļu".to_string();
        let result = get_first_result(&conn, &search_term).await.unwrap();
        assert_eq!(result.reg_code, reg_code, "Wrong search result.");
        // KATOĻU BAZNĪCAS RĒZEKNES
        let search_term = "KATOĻU BAZNĪCAS RĒZEKNES".to_string();
        let result = get_first_result(&conn, &search_term).await.unwrap();
        assert_eq!(result.reg_code, reg_code, "Wrong search result.");
        // KATOĻU
        let search_term = "KATOĻU".to_string();
        let result = get_first_result(&conn, &search_term).await.unwrap();
        assert_eq!(result.reg_code, reg_code, "Wrong search result.");
        // DIECĒZE
        let search_term = "DIECĒZE".to_string();
        let result = get_first_result(&conn, &search_term).await.unwrap();
        assert_eq!(result.reg_code, reg_code, "Wrong search result.");
        // not existing
        let search_term = "not existing".to_string();
        let result = get_first_result(&conn, &search_term).await;
        assert!(result.is_err(), "Should have not found.");
        // IS SIA
        let search_term = "Groglass".to_string();
        let result = get_first_result(&conn, &search_term).await.unwrap();
        assert_eq!(result.legal_form, "SIA".to_string());
        //HAS "-" IN NAME
        let search_term = "Med-Sea".to_string();
        let result = get_first_result(&conn, &search_term).await.unwrap();
        assert_eq!(result.reg_code, "50103563161".to_string());
        assert_eq!(result.legal_form, "SIA".to_string());
        //SIA iS REMOVED FROM BEGINNING, RETURNS FIRST COMPANY THAT HAS R
        let search_term = "SIA R".to_string();
        let result = get_first_result(&conn, &search_term).await.unwrap();
        assert_eq!(result.reg_code, "40008234596".to_string());
        // TBD AS Something Liepsaime
        let search_term = "AS Liepsaime".to_string();
        let result = get_first_result(&conn, &search_term).await.unwrap();
        println!("{:?}", result);
        assert_eq!(result.reg_code, "40203179017".to_string());
    }
}
