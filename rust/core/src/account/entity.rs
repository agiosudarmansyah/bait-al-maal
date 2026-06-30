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
    Bank(ProviderBank),
    EWallet(ProviderEWallet),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ProviderBank {
    BCA,
    BRI,
    BSI,
    BTN,
    Mandiri,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ProviderEWallet {
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