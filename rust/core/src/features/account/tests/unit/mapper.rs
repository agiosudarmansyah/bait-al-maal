use crate::account::AccountError;
use crate::account::tests::unit::helper::*;
use crate::infrastructure::database::mapper::account_mapper::{account_from_row, account_to_row};
use crate::shared::error::Error;

#[test]
fn account_to_row_maps_correctly() {
    let account = new_account();
    let account_row = account_to_row(&account);
    let id = account.id.as_bytes().to_vec();

    assert_eq!(account_row.id, id);
    assert_eq!(account_row.name, account.name);
    assert_eq!(account_row.icon_key, account.icon_key);
    assert_eq!(account_row.kind, "Cash");
    assert_eq!(account_row.provider, None);
    assert_eq!(account_row.amount, 100_000);
    assert_eq!(account_row.currency, "IDR");
}

#[test]
fn account_from_row_maps_correctly() {
    let account = new_account();
    let account_row = account_to_row(&account);

    let mapped = account_from_row(account_row).unwrap();

    assert_eq!(mapped.id, account.id);
    assert_eq!(mapped.name, account.name);
    assert_eq!(mapped.icon_key, account.icon_key);
    assert_eq!(mapped.account_type, account.account_type);
    assert_eq!(mapped.balance, account.balance);
}

#[test]
fn account_from_row_rejects_invalid_uuid() {
    let account = new_account();
    let mut account_row = account_to_row(&account);
    account_row.id.remove(0);

    let mapped = account_from_row(account_row);

    assert!(matches!(
        mapped,
        Err(Error::Account(AccountError::InvalidStoredData))
    ));
}
#[test]
fn account_from_row_rejects_invalid_kind() {
    let account = new_account();
    let mut account_row = account_to_row(&account);
    account_row.kind = String::from("Food");

    let mapped = account_from_row(account_row);

    assert!(matches!(
        mapped,
        Err(Error::Account(AccountError::InvalidStoredData))
    ));
}

#[test]
fn account_from_row_rejects_invalid_provider() {
    let account = new_account();
    let mut account_row = account_to_row(&account);
    account_row.provider = Some(String::from("Riverbank"));

    let mapped = account_from_row(account_row);

    assert!(matches!(
        mapped,
        Err(Error::Account(AccountError::InvalidStoredData))
    ));
}

#[test]
fn account_from_row_rejects_invalid_currency() {
    let account = new_account();
    let mut account_row = account_to_row(&account);
    account_row.currency = String::from("IRD");

    let mapped = account_from_row(account_row);

    assert!(matches!(
        mapped,
        Err(Error::Account(AccountError::InvalidStoredData))
    ));
}
