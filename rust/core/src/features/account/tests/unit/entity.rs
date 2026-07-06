use crate::account::{
    Account,
    AccountType,
    AccountError,
};
use crate::shared::money::{ Money, Currency };

fn account_new() -> Account {
    return Account::new(
        String::from("Aigo Cash"),
        String::from("Banknote"),
        AccountType::Cash,
        Money::new(100_000, Currency::IDR),
    )
}

// Constructor

#[test]
fn account_new_creates_account() {
    let account =  account_new();

    assert_eq!(account.name, "Aigo Cash");
    assert_eq!(account.icon_key, "Banknote");
    assert_eq!(account.account_type, AccountType::Cash);
    assert_eq!(account.balance.amount, 100_000);
    assert_eq!(account.balance.currency, Currency::IDR);
}

// Rename

#[test]
fn account_rename_changes_name() {
    let mut account = account_new();

    let result = account.rename(String::from("Aigo Wallet")) ;

    assert!(result.is_ok());
    assert_eq!(account.name, "Aigo Wallet");
}

#[test]
fn account_rename_rejects_empty_name() {
    let mut account = account_new();

    let result = account.rename(String::from(""));

    assert_eq!(result.unwrap_err(), AccountError::EmptyName);
}

#[test]
fn account_rename_does_not_change_name_on_error() {
    let mut account = account_new();

    let result = account.rename(String::from(""));

    assert!(result.is_err());
    assert_eq!(account.name, "Aigo Cash");
}

// Deposit

#[test]
fn account_deposit_increases_balance() {
    let mut account = account_new();

    let result = account.deposit(50_000);

    assert!(result.is_ok());
    assert_eq!(account.balance.amount, 150_000);
}

#[test]
fn account_deposit_rejects_zero() {
    let mut account = account_new();

    let result = account.deposit(0);

    assert_eq!(result.unwrap_err(), AccountError::AmountNotPositive);
}

#[test]
fn account_deposit_rejects_negative() {
    let mut account = account_new();

    let result = account.deposit(-10_000);

    assert_eq!(result.unwrap_err(), AccountError::AmountNotPositive);
}

#[test]
fn account_deposit_does_not_change_balance_on_error() {
    let mut account = account_new();

    let result = account.deposit(-10_000);

    assert!(result.is_err());
    assert_eq!(account.balance.amount, 100_000)
}

// Withdraw

#[test]
fn account_withdraw_decreases_balance() {
    let mut account = account_new();

    let result = account.withdraw(50_000);

    assert!(result.is_ok());
    assert_eq!(account.balance.amount, 50_000);
}

#[test]
fn account_withdraw_allows_zero_balance() {
    let mut account = account_new();

    let result = account.withdraw(100_000);

    assert!(result.is_ok());
    assert_eq!(account.balance.amount, 0);
}

#[test]
fn account_withdraw_rejects_zero() {
    let mut account = account_new();

    let result = account.withdraw(0);

    assert_eq!(result.unwrap_err(), AccountError::AmountNotPositive);
}

#[test]
fn account_withdraw_rejects_negative() {
    let mut account = account_new();

    let result = account.withdraw(-10_000);

    assert_eq!(result.unwrap_err(), AccountError::AmountNotPositive);
}

#[test]
fn account_withdraw_rejects_overdraft() {
    let mut account = account_new();

    let result = account.withdraw(110_000);

    assert_eq!(result.unwrap_err(), AccountError::NegativeBalance);
    assert_eq!(account.balance.amount, 100_000)
}

#[test]
fn account_withdraw_does_not_change_balance_on_error() {
    let mut account = account_new();

    let result = account.withdraw(150_000);

    assert!(result.is_err());
    assert_eq!(account.balance.amount, 100_000);
}

// Together

#[test]
fn account_deposit_then_withdraw_updates_balance_correctly() {
    let mut account = account_new();

    let result1 = account.deposit(100_000);
    let result2 = account.withdraw(30_000);

    assert!(result1.is_ok());
    assert!(result2.is_ok());
    assert_eq!(account.balance.amount, 170_000)
}

#[test]
fn account_multiple_deposits_accumulate_balance() {
    let mut account = account_new();

    let result1 = account.deposit(100_000);
    let result2 = account.deposit(50_000);
    let result3 = account.deposit(50_000);

    assert!(result1.is_ok());
    assert!(result2.is_ok());
    assert!(result3.is_ok());
    assert_eq!(account.balance.amount, 300_000)
}

#[test]
fn account_multiple_withdrawals_update_balance_correctly() {
    let mut account = account_new();

    let result1 = account.withdraw(30_000);
    let result2 = account.withdraw(30_000);
    let result3 = account.withdraw(30_000);

    assert!(result1.is_ok());
    assert!(result2.is_ok());
    assert!(result3.is_ok());
    assert_eq!(account.balance.amount, 10_000)
}