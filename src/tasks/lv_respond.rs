use crate::db::query::Query;
use crate::error::KybError;
use crate::tasks::lv_handle::handle_lv;
use actix_web::{HttpResponse, Responder, web};

pub async fn lv_respond(query: web::Json<Query>) -> impl Responder {
    match handle_lv(query).await {
        Ok(new_id) => {
            return HttpResponse::Ok()
                .json(serde_json::json!({ "valid": true, "verfication_id": new_id }));
        }
        Err(err) => match err {
            KybError::StringError(err) => {
                HttpResponse::ExpectationFailed().json(serde_json::json!({ "error": err }))
            }
            _ => {
                eprintln!("{:?}", err);
                HttpResponse::InternalServerError()
                    .json(serde_json::json!({ "error": "Server down" }))
            }
        },
    }
}
