use crate::db::get_db::Pool;
use crate::error::KybError;
use crate::latvia::board::query::Query;
use crate::tasks::lv_handle::handle_lv;
use actix_web::{HttpResponse, Responder, web};
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;

pub async fn lv_respond(pool: web::Data<Pool>, query: web::Json<Query>) -> impl Responder {
    let db: PooledConnection<SqliteConnectionManager> =
        pool.get().expect("Couldn't get db connection from pool");

    match handle_lv(&db, query).await {
        Ok(new_id) => {
            return HttpResponse::Ok()
                .json(serde_json::json!({ "valid": true, "verfication_id": new_id }));
        }
        Err(err) => match err {
            KybError::StringError(err) => {
                HttpResponse::ExpectationFailed().json(serde_json::json!({ "error": err }))
            }
            _ => {
                eprintln!("server down: {:?}", &err);
                HttpResponse::InternalServerError()
                    .json(serde_json::json!({ "error": "Server down" }))
            }
        },
    }
}
