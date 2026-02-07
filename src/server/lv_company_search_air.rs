use crate::{db::get_db::Pool, latvia::company::air::one_lv_air_message::one_lv_air_message};
use actix_web::{Error, HttpRequest, HttpResponse, rt, web};
use futures_util::StreamExt as _;

pub async fn lv_company_search_air(
    pool: web::Data<Pool>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;
    let mut stream = stream
        .aggregate_continuations()
        // aggregate continuation frames up to 10 kb
        .max_continuation_size(10 * 1024);
    let pool_clone = pool.clone();

    // start task but don't wait for it
    rt::spawn(async move {
        // receive messages from websocket
        while let Some(msg) = stream.next().await {
            if let Err(e) = one_lv_air_message(pool_clone.clone(), &mut session, msg).await {
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
    use crate::config::KybConfig;
    use crate::latvia::company::air::air_traffic::AirSearchResponse;
    use actix_web::Error;
    use futures_util::SinkExt;
    use futures_util::stream::StreamExt;
    use tokio::net::TcpStream;
    use tokio_tungstenite::tungstenite::Utf8Bytes;
    use tokio_tungstenite::tungstenite::protocol::Message;
    use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};

    async fn perform_air_search(
        ws_stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
        name: &str,
    ) -> Result<AirSearchResponse, Error> {
        let json_string = format!("{{\"name\": \"{}\"}}", name);
        let bytes = Utf8Bytes::from(json_string.clone());
        let msg = Message::Text(bytes);
        ws_stream.send(msg).await.expect("Unable to send message");

        if let Some(Ok(response)) = ws_stream.next().await {
            let payload: AirSearchResponse = serde_json::from_str(&response.to_string())?;
            Ok(payload)
        } else {
            panic!("No response received");
        }
    }

    // NB - THIS REQUIRES SERVER RUNNING
    #[actix_rt::test]
    async fn test_lv_company_search_air_single() {
        let url = format!("ws://{}/lv/air", KybConfig::SERVER_ADDRES);
        let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");

        let payload = perform_air_search(&mut ws_stream, "raimond fantastic")
            .await
            .unwrap();
        let container = payload.result.unwrap();
        let reg_code = container[0].clone().reg_code;
        assert_eq!(reg_code, "40203572370".to_string(), "Wrong search result.");
    }

    // NB - THIS REQUIRES SERVER RUNNING
    #[actix_rt::test]
    async fn test_lv_company_search_air_100_clients() {
        for _ in 0..100 {
            let url = format!("ws://{}/lv/air", KybConfig::SERVER_ADDRES);
            let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");

            let payload = perform_air_search(&mut ws_stream, "raimond fantastic")
                .await
                .unwrap();
            let container = payload.result.unwrap();
            let reg_code = container[0].clone().reg_code;
            assert_eq!(reg_code, "40203572370".to_string(), "Wrong search result.");
        }
    }
    // NB - THIS REQUIRES SERVER RUNNING
    #[actix_rt::test]
    async fn test_lv_company_search_air_1000_requests() {
        let url = format!("ws://{}/lv/air", KybConfig::SERVER_ADDRES);
        let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");

        for _ in 0..1000 {
            let payload = perform_air_search(&mut ws_stream, "raimond fantastic")
                .await
                .unwrap();
            let container = payload.result.unwrap();
            let reg_code = container[0].clone().reg_code;
            assert_eq!(reg_code, "40203572370".to_string(), "Wrong search result.");
        }
    }
}
