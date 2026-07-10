use thiserror::Error;

use crate::account::AccountError;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Account(#[from] AccountError),

    #[error(transparent)]
    Database(#[from] turso::Error),
}

pub type Result<T> = std::result::Result<T, AppError>;