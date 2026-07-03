use thiserror::Error;

/// Custom errors
#[derive(Error, Debug)]
pub enum AccountError {
    // Validation
    #[error("Invalid ID")]
    InvalidId,
    
    #[error("Account name cannot be empty")]
    EmptyName,

    #[error("Account balance cannot be negative")]
    NegativeBalance,


    // Lookup
    #[error("Account not found")]
    AccountNotFound,

    // Infrastructure
    #[error("Invalid stored data")]
    InvalidStoredData,
    
    #[error("Database error")]
    DatabaseError(#[from] turso::Error),
}

pub type Result<T> = std::result::Result<T, AccountError>;

