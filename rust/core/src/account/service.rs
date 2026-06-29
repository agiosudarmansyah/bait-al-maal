use crate::error::*;
use crate::{
    Account,
    AccountType::{self},
    AccountRepository,
    AccountRepositoryTrait,
};

pub struct AccountService {
    repository: AccountRepository
}

impl AccountService {
    pub async fn create (
        &self,
        name: String,
        icon_key: String,
        account_type: AccountType,
        balance: f64,
    ) -> Result<()> {
        let account = Account::new(
            name,
            icon_key, 
            account_type,
            balance,
        );

        self.repository.save(&account).await?;

        Ok(())
    }
}