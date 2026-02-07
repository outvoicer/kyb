use crate::config::KybConfig;
use crate::db::get_db::get_db;
use crate::server::api_test::api_test;
use crate::server::lv_board_respond::lv_board_respond;
use crate::server::lv_company_search::lv_company_search;
use crate::server::lv_company_search_air::lv_company_search_air;
use actix_web::{App, HttpServer, web};
use std::io::{Error, ErrorKind};

pub async fn start_server() -> std::io::Result<()> {
    match get_db() {
        Ok(db) => {
            let address = KybConfig::SERVER_ADDRES;
            println!("KYB server: {}", &address);
            //
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
        Err(err) => {
            eprintln!("Server down error: {}", err);
            let custom_error = Error::new(ErrorKind::Other, "Server down");
            return Err(custom_error);
        }
    }
}
