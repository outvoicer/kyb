use crate::config::KybConfig;
use crate::server::api_test::api_test;
use crate::server::lv_board_respond::lv_board_respond;
use crate::server::lv_company_search::lv_company_search;
use crate::server::lv_company_search_air::lv_company_search_air;
use actix_web::{App, HttpServer, web};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

pub async fn start_server(db: Pool<SqliteConnectionManager>) -> std::io::Result<()> {
    let address = KybConfig::SERVER_ADDRES;
    println!("KYB server: {}", &address);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/lv/board", web::post().to(lv_board_respond))
            .route("/lv/company", web::post().to(lv_company_search))
            .route("/lv/air", web::get().to(lv_company_search_air))
            .route("/api/test", web::get().to(api_test))
    })
    .bind(address)?
    .run()
    .await
}
