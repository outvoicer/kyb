use crate::{db::get_db::Pool, latvia::company::company::Company};
use actix_web::{HttpResponse, Responder, web};

pub async fn api_test(pool: web::Data<Pool>) -> impl Responder {
    let db = match pool.get() {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("Failed to get DB connection: {}", err);
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({ "error": "Failed to get DB connection" }));
        }
    };

    match Company::search_by_name(&db, &"Raimond Fantastic".to_string(), true).await {
        Ok(results) => {
            if let Some(first_result) = results.get(0) {
                if first_result.reg_code == "40203572370" {
                    HttpResponse::Ok().json(serde_json::json!({ "test": true }))
                } else {
                    HttpResponse::BadRequest()
                        .json(serde_json::json!({ "error": "DB empty maybe" }))
                }
            } else {
                HttpResponse::NotFound().json(serde_json::json!({ "error": "DB empty" }))
            }
        }
        Err(err) => {
            eprintln!("Search error: {}", err);
            HttpResponse::InternalServerError().json(serde_json::json!({ "error": "Server down" }))
        }
    }
}
