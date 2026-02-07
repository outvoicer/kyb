use crate::{db::get_db::Pool, latvia::company::company::Company};
use actix_web::{HttpResponse, Responder, web};
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;

pub async fn api_test(pool: web::Data<Pool>) -> impl Responder {
    // GET DB
    let db: PooledConnection<SqliteConnectionManager> =
        pool.get().expect("Couldn't get db connection from pool");
    // SEARCH FOR RAIMOND
    match Company::search_by_name(&db, &"Raimond Fantastic".to_string(), true).await {
        Ok(results) => {
            // MAKE SURE RAIMOND FANTASTIC DATA IS THERE
            if results[0].reg_code == "40203572370".to_string() {
                return HttpResponse::Ok().json(serde_json::json!({ "test": true }));
            } else {
                // RAIMOND IS MISSING
                return HttpResponse::InternalServerError()
                    .json(serde_json::json!({ "error": "Data missing" }));
            }
        }
        Err(err) => {
            // OTHER ERRORS WITH SEARCH
            eprintln!("test err: {}", err);
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({ "error": "Server down" }));
        }
    };
}
