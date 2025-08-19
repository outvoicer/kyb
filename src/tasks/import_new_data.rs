use crate::company::get_new_company_data::fetch_new_company_data;
use crate::db::get_db::get_db;
use crate::db::get_new_data::fetch_and_store_data;
use crate::public_institution::get_new_data::fetch_new_public_institution_data;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;

pub async fn import_new_data() {
    match get_db() {
        Ok(pool) => {
            // get db
            let mut conn: PooledConnection<SqliteConnectionManager> =
                pool.get().expect("Couldn't get db connection from pool");
            // GET MEMBERS OF BOARDS
            if let Err(e) = fetch_and_store_data(&mut conn).await {
                eprintln!("Error with member of board data: {}", e);
            }
            // GET COMPANIES
            if let Err(e) = fetch_new_company_data(&mut conn).await {
                eprintln!("Error with company data: {}", e);
            }
            // GET PUBLIC INSTITUTIONS
            if let Err(e) = fetch_new_public_institution_data(&mut conn).await {
                eprintln!("Error with public institutions data: {}", e);
            }
        }
        Err(err) => {
            eprintln!("{:?}", err);
        }
    }
}
