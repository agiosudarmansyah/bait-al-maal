use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;

use crate::account::{
    Account,
    AccountRepository,
};
use crate::infrastructure::database::{ 
    Database,
    mapper::account_mapper::{
        AccountRow,
        account_to_row,
        account_from_row,
    }
};
use crate::shared::error::{ Result };

pub struct SqliteAccountRepository {
    database: Arc<Database>
}

impl SqliteAccountRepository {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }
}

#[async_trait]
impl AccountRepository for SqliteAccountRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Account>> {
        let row = sqlx::query_as::<_, AccountRow>(
            r#"
            SELECT 
                id,
                name,
                icon_key,
                kind,
                provider,
                amount, 
                currrency
            FROM account
            WHERE id = ?
            "#
        )
        .bind(id.to_string())
        .fetch_optional(self.database.pool())
        .await?;

        let account_row = row
            .map(|row| account_from_row(row))
            .transpose();
    
        return account_row
    }

    async fn require_by_id(&self, id: Uuid) -> Result<Account> {
        let row = sqlx::query_as::<_, AccountRow>(
            r#"
            SELECT
                id,
                name,
                icon_key,
                kind,
                provider,
                amount,
                currency
            FROM account
            WHERE id = ?
            "#
        )
        .bind(id.to_string())
        .fetch_one(self.database.pool())
        .await?;

        Ok(account_from_row(row)?)
    }

    async fn require_all(&self) -> Result<Vec<Account>> {
        let rows = sqlx::query_as::<_, AccountRow>(
            r#"
            SELECT 
                id,
                name,
                icon_key,
                kind,
                provider,
                amount, 
                currrency
            FROM account
            "#
        )
        .fetch_all(self.database.pool())
        .await?;

        let accounts: Vec<Account> = rows
            .into_iter()
            .map(|row| account_from_row(row))
            .collect::<Result<Vec<_>>>()?;

        Ok(accounts)
    }

    async fn create(&self, account: &Account) -> Result<()> {
        let account_row = account_to_row(account);

        sqlx::query(
            r#"
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
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            "#
        )
        .bind(&account_row.id)
        .bind(&account_row.name)
        .bind(&account_row.icon_key)
        .bind(&account_row.kind)
        .bind(&account_row.provider)
        .bind(&account_row.amount)
        .bind(&account_row.currency)
        .execute(self.database.pool())
        .await?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM account 
            WHERE ID = ?
            "#
        )
        .bind(id.to_string())
        .execute(self.database.pool())
        .await?;

        Ok(())
    }
}