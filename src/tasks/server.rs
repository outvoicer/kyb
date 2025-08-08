use crate::tasks::handle_lv::handle_lv;
use actix_web::{App, HttpServer, web};

pub async fn start_server() -> std::io::Result<()> {
    let address = "127.0.0.1:9010";
    println!("KYB server: {}", &address);
    HttpServer::new(|| App::new().route("/lv", web::post().to(handle_lv)))
        .bind(address)?
        .run()
        .await
}
