use thiserror::Error;
use uuid;

/// Custom errors
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database connection failed")]
    DatabaseDisconnect(#[from] turso::Error),

    #[error("Uuid parse error")]
    UuidParseError(#[from] uuid::Error),

}

pub type Result<T> = std::result::Result<T, AppError>;

