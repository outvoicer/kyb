use crate::db::query::Query;
use crate::db::{get_db::get_db, log::log};
use crate::error::KybError;
use crate::verify::validate_and_verify::validate_and_verify;
use actix_web::{HttpResponse, Responder, web};

pub async fn handle_lv(query: web::Json<Query>) -> impl Responder {
    match get_db() {
        Ok(conn) => match validate_and_verify(&conn, &query).await {
            Ok(_) => {
                let new_id = log(&conn, &query, true).unwrap();
                HttpResponse::Ok()
                    .json(serde_json::json!({ "valid": true, "verfication_id": new_id }))
            }
            Err(err) => match err {
                KybError::StringError(err) => {
                    let e = format!("{:?}", err);
                    HttpResponse::ExpectationFailed().json(serde_json::json!({ "error": e }))
                }
                _ => {
                    eprintln!("{:?}", err);
                    HttpResponse::InternalServerError()
                        .json(serde_json::json!({ "error": "Server down" }))
                }
            },
        },
        Err(err) => HttpResponse::ExpectationFailed().json(serde_json::json!({ "error": "No db" })),
    }
}
