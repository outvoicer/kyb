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

    let result = Company::search_by_name(&db, &"Raimond Fantastic".to_string(), true).await;
    // drop(db);

    match result {
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

#[cfg(test)]
mod tests {
    use actix_web::test;
    use reqwest::Client;
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize)]
    pub struct TestResponse {
        pub test: bool,
    }

    #[test]
    async fn integration_lv_janis_is_member_of_board_100_times() {
        let client = Client::new();
        let url = "http://localhost:10001/api/test";

        for _ in 0..100 {
            let response = client
                .get(url)
                .header("Content-Type", "application/json")
                .send()
                .await
                .expect("Failed to send request");

            let text = response.text().await.expect("Failed to read response body");
            let resp: TestResponse = serde_json::from_str(&text).unwrap();
            assert_eq!(resp.test, true);
        }
    }
}
