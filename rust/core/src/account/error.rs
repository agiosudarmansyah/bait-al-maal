use thiserror::Error;
use uuid;

/// Custom errors
#[derive(Error, Debug)]
pub enum AccountError {
    // Repository errors
    #[error("Uuid parse error")]
    UuidParseError(#[from] uuid::Error),
    
    #[error("Uuid is missing or no input")]
    UuidNotFound,

    #[error("Database connection failed")]
    DatabaseDisconnect(#[from] turso::Error),
}

pub type Result<T> = std::result::Result<T, AccountError>;

