use crate::config::KybConfig;
use crate::tasks::handle_lv::handle_lv;
use actix_web::{App, HttpServer, web};

pub async fn start_server() -> std::io::Result<()> {
    let address = KybConfig::SERVER_ADDRES;
    println!("KYB server: {}", &address);
    HttpServer::new(|| App::new().route("/lv", web::post().to(handle_lv)))
        .bind(address)?
        .run()
        .await
}
