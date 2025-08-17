use std::error::Error as StdError;
use thiserror::Error;

//
#[derive(Error, Debug)]
pub enum KybError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),

    #[error("Error: {0}")]
    StringError(String),

    #[error("Std error: {0}")]
    StdError(#[from] Box<dyn StdError>),

    #[error("Serde JSON error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("Actix WS Closed error: {0:?}")]
    ActixWsClosedError(actix_ws::Closed),

    #[error("Actix Web error: {0}")]
    ActixWebError(#[from] actix_web::error::Error),
}
