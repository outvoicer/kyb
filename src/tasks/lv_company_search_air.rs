use crate::company::company::Company;
use crate::tasks::lv_company_search_handle::CompanySearchQuery;
use crate::tasks::lv_company_search_handle::lv_company_search_handle;
use actix_web::{Error, HttpRequest, HttpResponse, rt, web};
use actix_ws::AggregatedMessage;
use actix_ws::Session;
use futures_util::StreamExt as _;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct AirSearchResponse {
    result: Option<Vec<Company>>,
    error: Option<String>,
}

async fn send_message(session: &mut Session, input: AirSearchResponse) -> () {
    match serde_json::to_string(&input) {
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
                    // SERIALIZE
                    match serde_json::from_slice::<CompanySearchQuery>(&bytes.as_bytes()) {
                        Ok(query) => {
                            // SEARCH
                            match lv_company_search_handle(query).await {
                                Ok(result) => {
                                    // STRINGIFY:
                                    let respo = AirSearchResponse {
                                        result: Some(result),
                                        error: None,
                                    };
                                    match serde_json::to_string(&respo) {
                                        Ok(json_result) => {
                                            // SEND
                                            if let Err(e) = &session.text(json_result).await {
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

                            let respo = AirSearchResponse {
                                result: None,
                                error: Some("Did not understand message".to_string()),
                            };

                            let _ = send_message(&mut session, respo).await;
                        }
                    }
                }
                /*
                    Ok(AggregatedMessage::Binary(_bin)) => {
                        // echo binary message
                        if let Err(e) = session.text("Only text messages allowed").await {
                            println!("Failed to send message: {:?}", e);
                        }
                    }
                */
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::KybConfig;
    use actix_web::{App, test};
    use futures_util::SinkExt;
    use futures_util::stream::StreamExt;
    use tokio_tungstenite::connect_async;
    use tokio_tungstenite::tungstenite::Utf8Bytes;
    use tokio_tungstenite::tungstenite::protocol::Message;
    // TBD - THIS REQUIRES SERVER RINNING
    #[actix_rt::test]
    async fn test_lv_company_search_air() {
        let mut app =
            test::init_service(App::new().route("/lv/air", web::get().to(lv_company_search_air)))
                .await;
        // Connect to the WebSocket
        let url = format!("ws://{}/lv/air", KybConfig::SERVER_ADDRES);
        // Connect to the WebSocket server
        let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");
        let json_string = "{\"name\": \"raimond\"}";
        let a = Utf8Bytes::from_static(json_string);
        let msg = Message::Text(a);
        ws_stream.send(msg).await.expect("Unable to send message");

        // Optionally, read a response
        if let Some(Ok(response)) = ws_stream.next().await {
            println!("Received: {:?}", response);
        }

        // Check if the response is a successful WebSocket handshake
        // assert_eq!(response.status(), 101);
    }
}
