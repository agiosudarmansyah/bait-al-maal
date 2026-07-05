use async_trait::async_trait;
use uuid::Uuid;

use crate::account::{
    Account,
};
use crate::shared::error::AppError;

#[async_trait]
pub trait AccountRepository {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Account>, AppError>;
    async fn get_all(&self) -> Result<Vec<Account>, AppError>;
    async fn create(&self, account: &Account) -> Result<(), AppError>;
    async fn delete(&self, id: Uuid) -> Result<(), AppError>;
}
