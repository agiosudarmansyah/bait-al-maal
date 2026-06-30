use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;

use crate::account::entity::{ProviderBank, ProviderEWallet};
use crate:: error::*;
use crate::{
    Account,
    AccountType, 
    AppDatabase,
};

pub struct TursoAccountRepository {
    database: Arc<AppDatabase>
}

#[async_trait]
pub trait AccountRepository {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Account>>;
    async fn get_all(&self) -> Result<Vec<Account>>;
    async fn create(&self, account: &Account) -> Result<()>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}

fn account_type_to_db(account_type: &AccountType) -> (&'static str, Option<&'static str>) {
    match account_type {
        AccountType::Cash => ("Cash", None),
        AccountType::Bank(ProviderBank::BCA) => ("Bank", Some("BCA")),
        AccountType::Bank(ProviderBank::BRI) => ("Bank", Some("BRI")),
        AccountType::Bank(ProviderBank::BSI) => ("Bank", Some("BSI")),
        AccountType::Bank(ProviderBank::BTN) => ("Bank", Some("BTN")),
        AccountType::Bank(ProviderBank::Mandiri) => ("Bank", Some("Mandiri")),
        AccountType::EWallet(ProviderEWallet::Dana) => ("E-Wallet", Some("Dana")),
        AccountType::EWallet(ProviderEWallet::GoPay) => ("E-Wallet", Some("GoPay")),
        AccountType::EWallet(ProviderEWallet::Ovo) => ("E-Wallet", Some("Ovo")),
    }
}

// The return value Option<AccountType> should be revised as more items are added.

fn account_type_from_db(kind: &str, provider: Option<&str>) -> Option<AccountType> {
    match kind {
        "Cash" => Some(AccountType::Cash),
        "Bank" => match provider {
            Some("BCA") => Some(AccountType::Bank(ProviderBank::BCA)),
            Some("BRI") => Some(AccountType::Bank(ProviderBank::BRI)),
            Some("BSI") => Some(AccountType::Bank(ProviderBank::BSI)),
            Some("BTN") => Some(AccountType::Bank(ProviderBank::BTN)),
            Some("Mandiri") => Some(AccountType::Bank(ProviderBank::Mandiri)),
            _ => None,
        },
        "E-Wallet" => match provider {
            Some("Dana") => Some(AccountType::EWallet(ProviderEWallet::Dana)),
            Some("GoPay") => Some(AccountType::EWallet(ProviderEWallet::GoPay)),
            Some("Ovo") => Some(AccountType::EWallet(ProviderEWallet::Ovo)),
            _ => None,
        },
        _ => None,
    }
}

#[async_trait]
impl AccountRepository for TursoAccountRepository {

    // The variable account_type for read operations still uses unwrap() method which can cause panic, 
    // therefore proper error handling in the future is needed.

    async fn get_by_id(&self, id: Uuid) -> Result<Option<Account>> {
        let conn = self.database.connection()?;

        let mut rows = conn.query(
            "
            SELECT id, name, icon_key, kind, provider, balance FROM account
            WHERE id = ?
            ",
            (id.to_string(),)
        ).await?;

        if let Some(row) = rows.next().await? {
            let kind: String = row.get(3)?;
            let provider: Option<String> = row.get(4)?;

            let account_type = account_type_from_db(kind.as_str(), provider.as_deref());

            let account = Account {
                id: row.get::<String>(0)?.parse::<Uuid>()?,
                name: row.get(1)?,
                icon_key: row.get(2)?,
                account_type: account_type.unwrap(),
                balance: row.get(5)?,
            };

            return Ok(Some(account))
        }

        Ok(None)
    }
    
    async fn get_all(&self) -> Result<Vec<Account>> {
        let conn = self.database.connection()?;
        let mut accounts = Vec::new();
        
        let mut rows = conn.query(
            "
            SELECT id, name, icon_key, kind, provider, balance FROM account
            ", ()
        ).await?;
        
        while let Some(row) = rows.next().await? {
            let kind: String = row.get(3)?;
            let provider: Option<String> = row.get(4)?;

            let account_type = account_type_from_db(kind.as_str(), provider.as_deref());

            let account = Account {
                id: row.get::<String>(0)?.parse::<Uuid>()?,
                name: row.get(1)?,
                icon_key: row.get(2)?,
                account_type: account_type.unwrap(),
                balance: row.get(5)?,
            };

            accounts.push(account)
        }

        Ok(accounts)
    }

    async fn create(&self, account: &Account) -> Result<()> {
        let conn = self.database.connection()?;

        let (kind, provider) = account_type_to_db(&account.account_type);

        conn.execute(
            "
            INSERT INTO account
            (
                id,
                name,
                icon_key,
                kind,
                provider,
                balance
            )
            VALUES (?, ?, ?, ?, ?, ?)
            ", 
            (
                account.id.to_string(),
                account.name.as_str(),
                account.icon_key.as_str(),
                kind,
                provider,
                account.balance,
            ),   
        ).await?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
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