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
            // IS THERE FIRST RESULT
            if let Some(first_result) = results.get(0) {
                // IS IT RAIMOND FANTASTIC
                if first_result.reg_code == "40203572370" {
                    return HttpResponse::Ok().json(serde_json::json!({ "test": true }));
                } else {
                    // RAIMOND IS NOT THERE - DB IS EMPTY OR SOMETHING BAD HAS HAPPENED
                    return HttpResponse::BadRequest()
                        .json(serde_json::json!({ "error": "DB empty maybe" }));
                }
            } else {
                // MOST LIKELY EMPTY DB
                return HttpResponse::NotFound().json(serde_json::json!({ "error": "DB empty" }));
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
