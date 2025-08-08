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
}
