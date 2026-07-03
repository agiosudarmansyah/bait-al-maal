use async_trait::async_trait;
use uuid::Uuid;

use crate::account::{
    Account,
    Result,
};

#[async_trait]
pub trait AccountRepository {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Account>>;
    async fn get_all(&self) -> Result<Vec<Account>>;
    async fn create(&self, account: &Account) -> Result<()>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}
