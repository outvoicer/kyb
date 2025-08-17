use crate::config::KybConfig;
use crate::db::get_db::get_db;
use crate::tasks::lv_company_search::lv_company_search;
use crate::tasks::lv_company_search_air::lv_company_search_air;
use crate::tasks::lv_respond::lv_respond;
use actix_web::{App, HttpServer, web};
use std::io::{Error, ErrorKind};

pub async fn start_server() -> std::io::Result<()> {
    match get_db() {
        Ok(db) => {
            let address = KybConfig::SERVER_ADDRES;
            println!("KYB server: {}", &address);
            // let the_db = Mutex::new(db);
            HttpServer::new(move || {
                App::new()
                    .app_data(web::Data::new(db.clone()))
                    .route("/lv", web::post().to(lv_respond))
                    .route("/lv/company", web::post().to(lv_company_search))
                    .route("/lv/air", web::get().to(lv_company_search_air))
            })
            .bind(address)?
            .run()
            .await
        }
        Err(err) => {
            eprintln!("{}", err);
            let custom_error = Error::new(ErrorKind::Other, "Server down");
            return Err(custom_error);
        }
    }
}
