use crate::db::get_db::Pool;
use crate::error::KybError;
use crate::latvia::company::company::Company;
use crate::latvia::company::lv_company_search_handle::CompanySearchQuery;
use crate::latvia::company::lv_company_search_handle::lv_company_search_handle;
use actix_web::web;
use actix_ws::AggregatedMessage;
use actix_ws::ProtocolError;
use actix_ws::Session;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Ping {
    pub ping: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Pong {
    pub pong: bool,
}

pub async fn one_lv_air_message(
    pool: web::Data<Pool>,
    mut session: &mut Session,
    msg: Result<AggregatedMessage, ProtocolError>,
) -> Result<(), KybError> {
    // GET DB FOR SESSION
    let conn: PooledConnection<SqliteConnectionManager> =
        pool.get().expect("Couldn't get db connection from pool");
    // HANDLE MSG AND PING
    match msg {
        Ok(AggregatedMessage::Text(bytes)) => {
            // HANDLE ONE MESSAGE
            // SERIALIZE
            let as_v8 = bytes.as_bytes();
            let search = serde_json::from_slice::<CompanySearchQuery>(&as_v8);
            let ping = serde_json::from_slice::<Ping>(&as_v8);
            if search.is_ok() {
                let query = search.unwrap();
                let result = lv_company_search_handle(&conn, query).await?;
                // STRINGIFY:
                let respo = AirSearchResponse {
                    result: Some(result),
                    error: None,
                };
                // SEND
                send_message(&mut session, respo).await?;
                // Ok(())
            } else if ping.is_ok() {
                let pong = Pong { pong: true };
                let payload = serde_json::to_string(&pong)?;
                if let Err(e) = &session.text(payload).await {
                    return Err(KybError::StringError(e.to_string()));
                }
            } else {
                let respo = AirSearchResponse {
                    error: Some("Failed to deserialize message".to_string()),
                    result: None,
                };
                let _ = send_message(&mut session, respo).await;
            }
            /*
            match serde_json::from_slice::<CompanySearchQuery>(&bytes.as_bytes()) {
                Ok(query) => {
                    // SEARCH
                    let result = lv_company_search_handle(&conn, query).await?;
                    // STRINGIFY:
                    let respo = AirSearchResponse {
                        result: Some(result),
                        error: None,
                    };
                    // SEND
                    send_message(&mut session, respo).await?;
                    Ok(())
                }
                Err(_e) => {
                    let respo = AirSearchResponse {
                        result: None,
                        error: Some("Failed to deserialize message".to_string()),
                    };
                    let _ = send_message(&mut session, respo).await;
                    Ok(())
                }
            }
             */
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
    if let Err(e) = &session.text(payload).await {
        return Err(KybError::StringError(e.to_string()));
    }
    Ok(())
}
