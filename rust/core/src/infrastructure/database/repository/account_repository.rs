use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;

use crate::account::{
    Account,
    AccountRepository,
};
use crate::infrastructure::database::{ 
    AppDatabase,
    mapper::account_mapper::{
        account_to_row,
        account_from_row,
    }
};
use crate::shared::error::AppError;

pub struct TursoAccountRepository {
    database: Arc<AppDatabase>
}

#[async_trait]
impl AccountRepository for TursoAccountRepository {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Account>, AppError> {
        let conn = self.database.connection()?;

        let mut rows = conn.query(
            "
            SELECT id, name, icon_key, kind, provider, balance FROM account
            WHERE id = ?
            ",
            (id.to_string(),)
        ).await?;

        if let Some(row) = rows.next().await? {
            return Ok(Some(account_from_row(&row)?))
        }

        Ok(None)
    }
    
    async fn get_all(&self) -> Result<Vec<Account>, AppError> {
        let conn = self.database.connection()?;
        let mut accounts = Vec::new();
        
        let mut rows = conn.query(
            "
            SELECT id, name, icon_key, kind, provider, balance FROM account
            ", ()
        ).await?;
        
        while let Some(row) = rows.next().await? {
            let account = account_from_row(&row)?;

            accounts.push(account)
        }

        Ok(accounts)
    }

    async fn create(&self, account: &Account) -> Result<(), AppError> {
        let conn = self.database.connection()?;

        let (kind, provider, amount, currency) = account_to_row(account);

        conn.execute(
            "
            INSERT INTO account
            (
                id,
                name,
                icon_key,
                kind,
                provider,
                amount,
                currency
            )
            VALUES (?, ?, ?, ?, ?, ?, ?)
            ", 
            (
                account.id.to_string(),
                account.name.as_str(),
                account.icon_key.as_str(),
                kind,
                provider,
                amount,
                currency,
            ),   
        ).await?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let conn = self.database.connection()?;

        conn.execute(
            "
            DELETE FROM account 
            WHERE ID = ?
            ", (id.to_string(),)
        ).await?;

        Ok(())
    }
}