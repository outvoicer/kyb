use crate::company::company::Company;
use crate::error::KybError;
use crate::tasks::lv_company_search_handle::CompanySearchQuery;
use crate::tasks::lv_company_search_handle::lv_company_search_handle;
use actix_ws::AggregatedMessage;
use actix_ws::ProtocolError;
use actix_ws::Session;
use serde::{Deserialize, Serialize};

pub async fn one_lv_air_message(
    mut session: &mut Session,
    msg: Result<AggregatedMessage, ProtocolError>,
) -> Result<(), KybError> {
    match msg {
        Ok(AggregatedMessage::Text(bytes)) => {
            // SERIALIZE
            match serde_json::from_slice::<CompanySearchQuery>(&bytes.as_bytes()) {
                Ok(query) => {
                    // SEARCH
                    let result = lv_company_search_handle(query).await?;
                    // STRINGIFY:
                    let respo = AirSearchResponse {
                        result: Some(result),
                        error: None,
                    };
                    // SEND
                    send_message(&mut session, respo).await?;
                    Ok(())
                }
                Err(e) => {
                    // Deserialization failed
                    println!("Failed to deserialize message: {:?}", e);
                    let respo = AirSearchResponse {
                        result: None,
                        error: Some("Did not understand message".to_string()),
                    };
                    let _ = send_message(&mut session, respo).await;
                    Ok(())
                }
            }
        }

        Ok(AggregatedMessage::Ping(msg)) => {
            // respond to PING frame with PONG frame
            session.pong(&msg).await.unwrap();
            Ok(())
        }

        _ => Ok(()),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AirSearchResponse {
    result: Option<Vec<Company>>,
    error: Option<String>,
}

async fn send_message(session: &mut Session, input: AirSearchResponse) -> Result<(), KybError> {
    let payload = serde_json::to_string(&input)?;
    //session.text(payload).await;
    if let Err(e) = &session.text(payload).await {
        return Err(KybError::StringError(e.to_string()));
    }
    Ok(())
}
