use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
  pub id: Uuid,
  pub name: String,
  pub icon: String,
  pub account_type: AccountType,
  pub balance: i32,
}


/// enums need to be revised so it can be serialized/deserialized properly
#[derive(Debug, Serialize, Deserialize)]
pub enum AccountType {
    Cash,
    Bank(VariantBank),
    EWallet(VariantEWallet),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum VariantBank {
    BRI,
    BCA,
    Mandiri,
    BTN,
    BSI,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum VariantEWallet {
    GoPay,
    Dana,
    Ovo,
}

impl Account {
    pub fn new(
        id: Uuid,
        name: String,
        icon: String,
        account_type: AccountType,
        balance: i32,
    ) -> Self {
        Self {
            id,
            name,
            icon,
            account_type,
            balance,
        }
    }
}