use crate::tasks::lv_company_search_handle::CompanySearchQuery;
use crate::tasks::lv_company_search_handle::lv_company_search_handle;
use actix_web::{Error, HttpRequest, HttpResponse, rt, web};
use actix_ws::AggregatedMessage;
use futures_util::StreamExt as _;

pub async fn lv_company_search_air(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

    let mut stream = stream
        .aggregate_continuations()
        // aggregate continuation frames up to 10 kb
        .max_continuation_size(10 * 1024);

    // start task but don't wait for it
    rt::spawn(async move {
        // receive messages from websocket
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(AggregatedMessage::Text(bytes)) => {
                    println!("{}", &bytes);
                    // SERIALIZE
                    match serde_json::from_slice::<CompanySearchQuery>(&bytes.as_bytes()) {
                        Ok(query) => {
                            // SEARCH
                            match lv_company_search_handle(query).await {
                                Ok(result) => {
                                    // STRINGIFY:
                                    match serde_json::to_string(&result) {
                                        Ok(json_result) => {
                                            // SEND
                                            if let Err(e) = session.text(json_result).await {
                                                println!("Failed to send message: {:?}", e);
                                            }
                                        }
                                        Err(e) => {
                                            println!("Failed to serialize result: {:?}", e);
                                        }
                                    }
                                }
                                Err(err) => {}
                            }
                        }
                        Err(e) => {
                            // Deserialization failed
                            println!("Failed to deserialize message: {:?}", e);
                            // Handle the error, e.g., send an error response
                        }
                    }
                }

                Ok(AggregatedMessage::Binary(_bin)) => {
                    // echo binary message
                    if let Err(e) = session.text("Only text messages allowed").await {
                        println!("Failed to send message: {:?}", e);
                    }
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
}
