use crate::error::KybError;
use crate::latvia::company::company::Company;
use actix_ws::Session;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Ping {
    pub ping: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Pong {
    pub pong: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AirSearchResponse {
    pub result: Option<Vec<Company>>,
    pub error: Option<String>,
}

pub async fn send_message(session: &mut Session, input: AirSearchResponse) -> Result<(), KybError> {
    let payload = serde_json::to_string(&input)?;
    if let Err(e) = &session.text(payload).await {
        return Err(KybError::StringError(e.to_string()));
    }
    Ok(())
}
