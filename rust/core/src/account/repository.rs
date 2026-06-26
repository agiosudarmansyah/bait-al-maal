use uuid::Uuid;
use async_trait::async_trait;

use crate::{AppDatabase, account};
use crate::{Account, error::AppError};

#[async_trait]
pub trait AccountRepository {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Account>, AppError>;
    async fn get_all(&self) -> Result<Vec<Account>, AppError>;
    async fn save(&self, account: &Account) -> Result<(), AppError>;
    async fn delete(&self, id: Uuid) -> Result<(), AppError>;
}

// Intentionally left empty as temporary means...

// #[async_trait]
// impl AccountRepository for Account {
//     async fn get_by_id(&self, id: Uuid) -> Result<Option<Account>, AppError> {
//         let id = Account::;
//     }

//     async fn get_all(&self) -> Result<Vec<Account>, AppError> {

        
//     }

//     async fn save(&self, account: &Account) -> Result<(), AppError> {
        
//     }

//     async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        
//     }
// }