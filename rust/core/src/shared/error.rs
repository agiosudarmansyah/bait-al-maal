use thiserror::Error;

use crate::account::AccountError;

#[derive(Debug, Error)]
pub enum Error {
    // Features
    #[error(transparent)]
    Account(#[from] AccountError),

    //Infrastructure
    #[error("Requested resource was not found")]
    NotFound,

    #[error(transparent)]
    Database(#[from] sqlx::Error),

    // Other
    #[error(transparent)]
    Env(#[from] std::env::VarError),
}

pub type Result<T> = std::result::Result<T, Error>;