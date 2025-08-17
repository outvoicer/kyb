use crate::db::query::Query;
use crate::error::KybError;
use crate::verify::is_board_member::is_board_member;
use crate::verify::validate::validate_data;
use rusqlite::Connection;

pub async fn validate_and_verify(conn: &Connection, query: &Query) -> Result<(), KybError> {
    validate_data(query)?;
    is_board_member(&conn, query).await?;
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

        let q = Query {
            name: "Familiname Inese".to_string(),
            reg_code: "40008093564".to_string(),
            personal_code: "140777-*****".to_string(),
        };

        let _q2 = Query::new(
            "Familiname Inese".to_string(),
            "40008093564".to_string(),
            "40008093564".to_string(),
        );

        let is = validate_and_verify(&conn, &q).await;
        assert!(is.is_ok());
    }

    #[test]
    async fn test_is_officer_failure() {
        let conn = setup_test_db().await;

        let q = Query {
            name: "Nonexistent Name".to_string(),
            reg_code: "000000".to_string(),
            personal_code: "00000000000-*****".to_string(),
        };

        let is = validate_and_verify(&conn, &q).await;
        assert!(is.is_err());
    }

    #[test]
    async fn test_correct_janis() {
        let conn = setup_test_db().await;

        let q = Query {
            name: "Bērziņš Jānis".to_string(),
            reg_code: "40103254484".to_string(),
            personal_code: "240655-*****".to_string(),
        };

        let is = validate_and_verify(&conn, &q).await;
        assert!(is.is_ok());
    }

    #[test]
    async fn test_two_janises() {
        let conn = setup_test_db().await;

        let q = Query {
            name: "Bērziņš Jānis".to_string(),
            reg_code: "40103235360".to_string(),
            personal_code: "240655-*****".to_string(),
        };

        let is = validate_and_verify(&conn, &q).await;
        assert!(is.is_err());
    }

    #[test]
    async fn test_joonas_without_personal_code_in_db_is_valid() {
        let conn = setup_test_db().await;

        let q = Query {
            name: "Test Joonas".to_string(),
            reg_code: "40103235360".to_string(),
            personal_code: "12345678".to_string(),
        };

        let is = validate_and_verify(&conn, &q).await;
        assert!(is.is_ok());
    }

    #[test]
    async fn regular_janis_instead_of_jānis_fails() {
        let conn = setup_test_db().await;

        let q = Query {
            name: "Bērziņš Janis".to_string(),
            reg_code: "40103254484".to_string(),
            personal_code: "240655-*****".to_string(),
        };

        let is = validate_and_verify(&conn, &q).await;
        assert!(is.is_err());
    }

    #[test]
    async fn test_is_stars_fails() {
        let conn = setup_test_db().await;

        let q = Query {
            name: "*".to_string(),
            reg_code: "*".to_string(),
            personal_code: "*".to_string(),
        };

        let is = validate_and_verify(&conn, &q).await;
        assert!(is.is_err());
    }
}
