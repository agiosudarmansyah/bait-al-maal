pub mod features;
pub mod infrastructure;
pub mod shared;

pub use features::*;
pub use crate::infrastructure::database::Database;

pub struct Startup {
    database: Arc<Database>
}

use std::sync::Arc;

impl Startup {
    fn new(database: Arc<Database>) -> Self {
        Self { database }
    }

    async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        const TABLES: [&str; 4] = [
            include_str!("infrastructure/database/schemas/account.sql"),
            include_str!("infrastructure/database/schemas/budget.sql"),
            include_str!("infrastructure/database/schemas/category.sql"),
            include_str!("infrastructure/database/schemas/transaction.sql")
        ];

        for table in TABLES {
            sqlx::query(table)
            .execute(self.database.pool())
            .await?;
        }

        Ok(())
    }
}

async fn startup(database: Arc<Database>) -> Result<(), Box<dyn std::error::Error>> {
    Startup::new(database).run().await
}