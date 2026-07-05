use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::account::{ AccountError, Result };
use crate::shared::money::Money;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
  pub id: Uuid,
  pub name: String,
  pub icon_key: String,
  pub account_type: AccountType,
  pub balance: Money,
}


/// enums need to be revised so it can be serialized/deserialized properly
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
pub enum AccountType {
    Cash,
    Bank(ProviderBank),
    EWallet(ProviderEWallet),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
pub enum ProviderBank {
    BCA,
    BRI,
    BSI,
    BTN,
    Mandiri,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
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
        balance: Money,
    ) -> Self {
        Self {
            id: Uuid::now_v7(),
            name,
            icon_key,
            account_type,
            balance
        }
    }

    pub fn rename(&mut self, new_name: String) -> Result<()> {
        if new_name == "" {
            return Err(AccountError::EmptyName)
        }

        self.name = new_name;

        Ok(())
    }

    pub fn deposit(&mut self, deposit: i64) -> Result<()> {
        if deposit <= 0 {
            return Err(AccountError::AmountNotPositive)
        }

        self.balance.amount += deposit;

        Ok(())
    }

    pub fn withdraw(&mut self, withdraw: i64) -> Result<()> {
        if withdraw <= 0 {
            return Err(AccountError::AmountNotPositive)
        } else if self.balance.amount < withdraw {
            return Err(AccountError::NegativeBalance)
        } else {
            self.balance.amount -= withdraw
        }

        Ok(())
    }
}