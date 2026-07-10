use thiserror::Error;

/// Custom errors
#[derive(Error, Debug, Eq, PartialEq)]
pub enum AccountError {
    // Validation
    #[error("Invalid ID")]
    InvalidId,
    
    #[error("Account name cannot be empty")]
    EmptyName,

    #[error("Passed amount must not be negative or zero")]
    AmountNotPositive,

    #[error("Account balance cannot be negative")]
    NegativeBalance,


    // Lookup
    #[error("Account not found")]
    AccountNotFound,

    // Infrastructure
    #[error("Invalid stored data")]
    InvalidStoredData,
}

pub type AccountResult<T> = std::result::Result<T, AccountError>;

