use anyhow::{ anyhow, bail,  Result, };
use uuid::Uuid;

use crate::{
    Account, 
    AccountRepository, 
    AccountType::{self}, 
    TursoAccountRepository,
};

pub struct AccountService {
    repository: TursoAccountRepository
}

impl AccountService {
    pub fn new(repository: TursoAccountRepository) -> Self {
        Self { repository }
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Account> {
        if id.is_nil() {
            bail!("Required UUID is missing");
        }

        let account = self
            .repository
            .get_by_id(id)
            .await?
            .ok_or_else(|| anyhow!("No account was found"))?;

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
            bail!("Required UUID is missing")
        }

        self.repository.delete(id).await?;

        Ok(())
    }
}