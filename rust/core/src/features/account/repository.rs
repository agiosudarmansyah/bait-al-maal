use async_trait::async_trait;
use mockall::automock;
use uuid::Uuid;

use crate::account::{
    Account,
};
use crate::shared::error::Result;

#[automock]
#[async_trait]
pub trait AccountRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Account>>;
    async fn require_by_id(&self, id: Uuid) -> Result<Account>;
    async fn require_all(&self) -> Result<Vec<Account>>;
    async fn create(&self, account: &Account) -> Result<()>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}
