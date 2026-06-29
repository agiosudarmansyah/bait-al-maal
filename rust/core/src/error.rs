use thiserror::Error;
use uuid::Uuid;

/// Custom errors
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Error when fetching specified ID: {id}: {reason}")]
    IdNotFound { id: Uuid, reason: String },

    #[error("Database connection failed")]
    DatabaseDisconnect(#[from] turso::Error),
}

pub type Result<T> = std::result::Result<T, AppError>;

