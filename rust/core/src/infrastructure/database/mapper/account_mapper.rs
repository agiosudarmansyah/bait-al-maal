use uuid::Uuid;

use crate::account::{
    Account,
    AccountError,
    AccountType, 
    ProviderBank,
    ProviderEWallet,
};
use crate::shared::{
    error::{ Result },
    money::{ Money, Currency },
    sql_enum::SqlEnum,
};

#[derive(sqlx::FromRow)]
pub struct AccountRow {
    pub id: Vec<u8>,
    pub name: String,
    pub icon_key: String,
    pub kind: String,
    pub provider: Option<String>,
    pub amount: i64,
    pub currency: String,
}

pub fn account_type_to_sql(account_type: &AccountType) -> (&'static str, Option<&'static str>) {
    match account_type {
        AccountType::Cash => ("Cash", None),
        AccountType::Bank(ProviderBank::BCA) => ("Bank", Some("BCA")),
        AccountType::Bank(ProviderBank::BRI) => ("Bank", Some("BRI")),
        AccountType::Bank(ProviderBank::BSI) => ("Bank", Some("BSI")),
        AccountType::Bank(ProviderBank::BTN) => ("Bank", Some("BTN")),
        AccountType::Bank(ProviderBank::Mandiri) => ("Bank", Some("Mandiri")),
        AccountType::EWallet(ProviderEWallet::Dana) => ("E-Wallet", Some("Dana")),
        AccountType::EWallet(ProviderEWallet::GoPay) => ("E-Wallet", Some("GoPay")),
        AccountType::EWallet(ProviderEWallet::Ovo) => ("E-Wallet", Some("Ovo")),
    }
}

// Consider refactoring this function below to return value Result<AccountType>.

pub fn account_type_from_sql(kind: &str, provider: Option<&str>) -> Option<AccountType> {
    match kind {
        "Cash" => Some(AccountType::Cash),
        "Bank" => match provider {
            Some("BCA") => Some(AccountType::Bank(ProviderBank::BCA)),
            Some("BRI") => Some(AccountType::Bank(ProviderBank::BRI)),
            Some("BSI") => Some(AccountType::Bank(ProviderBank::BSI)),
            Some("BTN") => Some(AccountType::Bank(ProviderBank::BTN)),
            Some("Mandiri") => Some(AccountType::Bank(ProviderBank::Mandiri)),
            _ => None,
        },
        "E-Wallet" => match provider {
            Some("Dana") => Some(AccountType::EWallet(ProviderEWallet::Dana)),
            Some("GoPay") => Some(AccountType::EWallet(ProviderEWallet::GoPay)),
            Some("Ovo") => Some(AccountType::EWallet(ProviderEWallet::Ovo)),
            _ => None,
        },
        _ => None,
    }
}

pub fn money_to_sql (money: &Money) -> (i64, &'static str) {
    (money.amount, money.currency.to_sql())
}

pub fn money_from_sql(amount: i64, currency: &str) -> Result<Money> {
    let currency = Currency::from_sql(currency).ok_or(AccountError::InvalidStoredData)?;
    Ok(Money::new(amount, currency))
}

pub fn account_to_row(account: &Account) -> AccountRow {
    let (kind, provider) = account_type_to_sql(&account.account_type);
    let (amount, currency) = money_to_sql(&account.balance);

    AccountRow { 
        id: account.id.as_bytes().to_vec(), 
        name: account.name.to_string(), 
        icon_key: account.icon_key.to_string(), 
        kind: kind.to_string(), 
        provider: provider.map(str::to_string), 
        amount, 
        currency: currency.to_string(), 
    }
}

pub fn account_from_row(row: AccountRow) -> Result<Account> {
    let account_type = account_type_from_sql(
        &row.kind,
        row.provider.as_deref(),
    )
    .ok_or(AccountError::InvalidStoredData)?;

    let balance = money_from_sql(
        row.amount,
        &row.currency,
    )?;

    Ok(Account {
        id: Uuid::from_slice(&row.id).map_err(|_| AccountError::InvalidStoredData)?,
        name: row.name,
        icon_key: row.icon_key,
        account_type,
        balance,
    })
}