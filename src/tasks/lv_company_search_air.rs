use crate::error::KybError;
use crate::tasks::lv_company_search_handle::CompanySearchQuery;
use crate::tasks::lv_company_search_handle::lv_company_search_handle;
use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, rt, web};
use actix_ws::AggregatedMessage;
use futures_util::StreamExt as _;
//use futures_util::stream::stream::StreamExt;

//async fn echo(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {

//query: web::Json<CompanySearchQuery>
pub async fn lv_company_search_air(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

    let mut stream = stream
        .aggregate_continuations()
        // aggregate continuation frames up to 1MiB
        .max_continuation_size(2_usize.pow(20));

    // start task but don't wait for it
    rt::spawn(async move {
        // receive messages from websocket
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(AggregatedMessage::Text(text)) => {
                    // echo text message
                    session.text(text).await.unwrap();
                }

                Ok(AggregatedMessage::Binary(bin)) => {
                    // echo binary message
                    session.binary(bin).await.unwrap();
                }

                Ok(AggregatedMessage::Ping(msg)) => {
                    // respond to PING frame with PONG frame
                    session.pong(&msg).await.unwrap();
                }

                _ => {}
            }
        }
    });

    // respond immediately with response connected to WS session
    Ok(res)

    /*
    match lv_company_search_handle(query).await {
        Ok(results) => {
            return HttpResponse::Ok().json(results);
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
    */
}
