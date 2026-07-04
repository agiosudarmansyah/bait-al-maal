use uuid::Uuid;

use crate::account::{
    Account,
    AccountError,
    AccountType, 
    ProviderBank,
    ProviderEWallet,
    Result,
};
use crate::shared::{
    money::{ Money, Currency },
    sql_enum::SqlEnum,
};

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

pub fn account_to_row(account: &Account) -> (&str, Option<&str>, i64, &'static str) {
    let (kind, provider) = account_type_to_sql(&account.account_type);
    let (amount, currency) = money_to_sql(&account.balance);

    (kind, provider, amount, currency)
}

pub fn account_from_row(row: &turso::Row) -> Result<Account> {
    let kind: String = row.get(3)?;
    let provider: Option<String> = row.get(4)?;
    let account_type = account_type_from_sql(kind.as_str(), provider.as_deref());

    let amount: i64 = row.get(5)?;
    let currency: String = row.get(6)?;
    let balance = money_from_sql(amount, &currency)?;

    let account = Account {
        id: row.get::<String>(0)?.parse::<Uuid>().map_err(|_| AccountError::InvalidStoredData)?,
        name: row.get(1)?,
        icon_key: row.get(2)?,
        account_type: account_type.ok_or(AccountError::InvalidStoredData)?,
        balance: balance,
    };

    Ok(account)
}