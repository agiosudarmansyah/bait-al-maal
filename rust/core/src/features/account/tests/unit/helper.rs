use std::sync::Arc;

use crate::{Database};
use crate::account::{
    Account,
    AccountType,
};
use crate::infrastructure::database::repository::account_repository::SqliteAccountRepository;
use crate::shared::money::{ Money, Currency };

pub fn new_account() -> Account {
    return Account::new(
        String::from("Aigo Cash"),
        String::from("Banknote"),
        AccountType::Cash,
        Money::new(100_000, Currency::IDR),
    )
}

pub struct TestContext {
    pub db: Arc<Database>,
    pub repo: SqliteAccountRepository,
}

pub async fn setup() -> TestContext {
    let db = Arc::new(Database::memory().await.unwrap());
    let repo = SqliteAccountRepository::new(db.clone());

    TestContext { db, repo }
}