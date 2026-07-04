use uuid::Uuid;

use crate::account::{
    Account, 
    AccountError,
    AccountRepository, 
    AccountType::{self}, 
    Result,
};

pub struct AccountService<R>
where 
    R: AccountRepository
{
    repository: R
}

impl<R> AccountService<R>
where
    R: AccountRepository
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Account> {
        if id.is_nil() {
            return Err(AccountError::InvalidId)
        }

        let account = self
            .repository
            .get_by_id(id)
            .await?
            .ok_or(AccountError::AccountNotFound)?;

        Ok(account)
    }

    pub async fn get_all(&self) -> Result<Vec<Account>> {
        let account = self.repository.get_all().await?;

        Ok(account)
    }

    pub async fn create(
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

        self.repository.create(&account).await?;

        Ok(())
    }

    pub async fn delete(&self, id: Uuid) -> Result<()> {
        if id.is_nil() {
            return Err(AccountError::InvalidId)
        }

        self.repository.delete(id).await?;

        Ok(())
    }
}