use std::sync::Arc;
use async_trait::async_trait;

use crate::error::*;
use crate::{
    Account, 
    AppDatabase,
};

pub struct AccountRepository {
    database: Arc<AppDatabase>
}

#[async_trait]
pub trait AccountRepositoryTrait {
    async fn save(&self, account: &Account) -> Result<()>;
    // async fn get_by_id(&self, id: Uuid) -> Result<Option<Account>>;
    // async fn get_all(&self) -> Result<Vec<Account>>;
    // async fn delete(&self, id: Uuid) -> Result<()>;
}

// Intentionally left empty as temporary means...

#[async_trait]
impl AccountRepositoryTrait for AccountRepository {
    async fn save(&self, account: &Account) -> Result<()> {
        let conn = self.database.connection()?;

        conn.execute(
            "
            INSERT INTO account
            (
                id,
                name,
                icon,
                account_type,
                balance
            )
            VALUES (?, ?, ?, ?, ?)
            ", 
            (
                account.id.to_string(),
                account.name.as_str(),
                account.icon_key.as_str(),
                account.account_type.to_string(),
                account.balance,
            ),   
        ).await?;

        Ok (())
    }

    // async fn get_by_id(&self, id: Uuid) -> Result<Option<Account>> {
    //     let id = Account::;
    // }

    // async fn get_all(&self) -> Result<Vec<Account>, AppError> {

        
    // }


    // async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        
    // }
}