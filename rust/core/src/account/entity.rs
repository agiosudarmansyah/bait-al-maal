use std::fmt;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
  pub id: Uuid,
  pub name: String,
  pub icon_key: String,
  pub account_type: AccountType,
  pub balance: f64,
}


/// enums need to be revised so it can be serialized/deserialized properly
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AccountType {
    Cash,
    Bank(Bank),
    EWallet(EWallet),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Bank {
    BCA,
    BRI,
    BSI,
    BTN,
    Mandiri,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EWallet {
    Dana,
    GoPay,
    Ovo,
}

impl Account {
    pub fn new(
        name: String,
        icon_key: String,
        account_type: AccountType,
        balance: f64,
    ) -> Self {
        Self {
            id: Uuid::now_v7(),
            name,
            icon_key,
            account_type,
            balance
        }
    }
}

impl fmt::Display for AccountType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cash => write!(f, "cash"),
            Self::Bank(bank) => match bank {
                &Bank::BCA => write!(f, "{}, {}", "Bank", "BCA"),
                &Bank::BRI => write!(f, "{}, {}", "Bank", "BRI"),
                &Bank::BSI => write!(f, "{}, {}", "Bank", "BSI"),
                &Bank::BTN => write!(f, "{}, {}", "Bank", "BTN"),
                &Bank::Mandiri => write!(f, "{}, {}", "Bank", "Mandiri"),
            },
            Self::EWallet(e_wallet) => match e_wallet {
                &EWallet::Dana => write!(f, "{}, {}", "E-Wallet", "Dana"),
                &EWallet::GoPay => write!(f, "{}, {}", "E-Wallet", "GoPay"),
                &EWallet::Ovo => write!(f, "{}, {}", "E-Wallet", "Ovo"),
            }
        }
    }
}