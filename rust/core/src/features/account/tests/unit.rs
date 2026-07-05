use crate::account::{
    Account,
    AccountType,
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

// Entity
#[test]
fn account_new_creates_account() {
    let account =  account_new();

    assert_eq!(account.name, "Aigo Cash");
    assert_eq!(account.icon_key, "Banknote");
    assert_eq!(account.account_type, AccountType::Cash);
    assert_eq!(account.balance.amount, 100_000);
    assert_eq!(account.balance.currency, Currency::IDR);
}

#[test]
fn account_rename_changes_name() {
    let mut account = account_new();
    account.name = String::from("Aigo Wallet");

    assert_eq!(account.name, "Aigo Wallet");
}

#[test]
fn account_deposit_increases_balance() {
    let mut account = account_new();
    let deposit = 50_000;
    account.balance.amount = account.balance.amount + deposit;

    assert_eq!(account.balance.amount, 150_000);
}

#[test]
fn account_withdraw_decreases_balance() {
    let mut account = account_new();
    let withdraw = 50_000;
    account.balance.amount = account.balance.amount - withdraw;

    assert_eq!(account.balance.amount, 50_000);
}

// Repository

// Service