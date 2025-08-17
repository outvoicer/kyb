use crate::tasks::one_lv_air_message::one_lv_air_message;
use actix_web::{Error, HttpRequest, HttpResponse, rt, web};
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
            if let Err(e) = one_lv_air_message(&mut session, msg).await {
                //return Err(e);
                println!("LV air message error: {}", e);
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
