use crate::latvia::board::create_table::create_table;
use rusqlite::{Connection, params};

#[allow(dead_code)]
pub async fn setup_test_db() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    let _ = create_table(&conn).await;
    conn.execute(
        "INSERT INTO officers (reg_code, name, personal_code) VALUES (?1, ?2, ?3)",
        params!["40008093564", "Familiname Inese", "140777-*****"],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO officers (reg_code, name, personal_code) VALUES (?1, ?2, ?3)",
        params!["40103254484", "Bērziņš Jānis", "240655-*****"],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO officers (reg_code, name, personal_code) VALUES (?1, ?2, ?3)",
        params!["40103235360", "Bērziņš Jānis", "201292-*****"],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO officers (reg_code, name, personal_code) VALUES (?1, ?2, ?3)",
        params!["40103235360", "Test Joonas", ""],
    )
    .unwrap();

    conn
}
