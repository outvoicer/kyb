use crate::db::query::query_officer;
use actix_web::{App, HttpServer, web};

pub async fn start_server() -> std::io::Result<()> {
    let address = "127.0.0.1:8080";
    println!("KYB server: {}", &address);
    HttpServer::new(|| App::new().route("/lv", web::post().to(query_officer)))
        .bind(address)?
        .run()
        .await
}
