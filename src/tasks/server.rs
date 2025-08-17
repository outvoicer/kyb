use crate::config::KybConfig;
use crate::tasks::lv_company_search::lv_company_search;
use crate::tasks::lv_company_search_air::lv_company_search_air;
use crate::tasks::lv_respond::lv_respond;
use actix_web::{App, HttpServer, web};

pub async fn start_server() -> std::io::Result<()> {
    let address = KybConfig::SERVER_ADDRES;
    println!("KYB server: {}", &address);
    HttpServer::new(|| {
        App::new()
            .route("/lv", web::post().to(lv_respond))
            .route("/lv/company", web::post().to(lv_company_search))
            .route("/lv/air", web::get().to(lv_company_search_air))
    })
    .bind(address)?
    .run()
    .await
}
