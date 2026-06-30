use crate::error::*;
use crate::{
    Account,
    AccountType::{self},
    AccountRepository,
    TursoAccountRepository,
};

pub struct AccountService {
    repository: TursoAccountRepository
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

        self.repository.create(&account).await?;

        Ok(())
    }
}