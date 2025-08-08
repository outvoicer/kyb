use crate::db::log::log;
use crate::verify::validate_and_verify::validate_and_verify;
use actix_web::{HttpResponse, Responder, web};
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Query {
    pub name: String,
    pub personal_code: String,
    pub reg_code: String,
}

impl Query {
    pub fn new(name: String, personal_code: String, reg_code: String) -> Self {
        Self {
            name: name,
            personal_code: personal_code,
            reg_code: reg_code,
        }
    }
}

pub async fn query_officer(query: web::Json<Query>) -> impl Responder {
    let conn = Connection::open("officers.db").unwrap();
    match validate_and_verify(&conn, &query).await {
        Ok(_) => {
            let new_id = log(&conn, &query, true).unwrap();
            HttpResponse::Ok().json(serde_json::json!({ "valid": true, "verfication_id": new_id }))
        }
        Err(err) => HttpResponse::ExpectationFailed().json(serde_json::json!({ "error": err })),
    }
}
