use crate::db::get_db::Pool;
use crate::error::KybError;
use crate::latvia::board::lv_board_handle::lv_board_handle;
use crate::latvia::board::query::Query;
use actix_web::{HttpResponse, Responder, web};
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;

pub async fn lv_board_respond(pool: web::Data<Pool>, query: web::Json<Query>) -> impl Responder {
    // GET DB
    let db: PooledConnection<SqliteConnectionManager> =
        pool.get().expect("Couldn't get db connection from pool");

    match lv_board_handle(&db, query).await {
        Ok(new_id) => {
            //drop(pool);
            return HttpResponse::Ok()
                .json(serde_json::json!({ "valid": true, "verfication_id": new_id }));
        }
        Err(err) => match err {
            KybError::StringError(err) => {
                //drop(pool);
                HttpResponse::ExpectationFailed().json(serde_json::json!({ "error": err }))
            }
            _ => {
                // drop(pool);
                eprintln!("server down: {:?}", &err);
                HttpResponse::InternalServerError()
                    .json(serde_json::json!({ "error": "Server down" }))
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use actix_web::test;
    use reqwest::Client;
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    #[derive(Deserialize, Serialize)]
    pub struct BoardResponse {
        pub valid: bool,
        pub verfication_id: u64,
    }

    #[test]
    async fn integration_lv_janis_is_member_of_board_1000_times() {
        let client = Client::new();
        let url = "http://localhost:10001/lv/board";
        let payload = json!({ "name": "Bērziņš Jānis", "personal_code": "201292-*****", "reg_code": "40103235360" });

        for _ in 0..1000 {
            let response = client
                .post(url)
                .header("Content-Type", "application/json")
                .body(payload.to_string())
                .send()
                .await
                .expect("Failed to send request");

            let text = response.text().await.expect("Failed to read response body");
            let resp: BoardResponse = serde_json::from_str(&text).unwrap();
            assert_eq!(resp.valid, true);
        }
    }
}
