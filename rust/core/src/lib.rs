pub mod features;
pub mod infrastructure;
pub mod shared;

pub mod app_state;

pub use features::*;
pub use crate::infrastructure::database::AppDatabase;

pub struct Startup {
    database: Arc<AppDatabase>
}

use std::sync::Arc;

impl Startup {
    fn new(database: Arc<AppDatabase>) -> Self {
        Self { database }
    }

    async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.database.connection()?;
    
        const TABLES: [&str; 4] = [
            include_str!("infrastructure/database/schemas/account.sql"),
            include_str!("infrastructure/database/schemas/budget.sql"),
            include_str!("infrastructure/database/schemas/category.sql"),
            include_str!("infrastructure/database/schemas/transaction.sql")
        ];

        for table in TABLES {
            conn.execute(table, ()).await?;
        }

        Ok(())
    }
}

async fn startup(database: Arc<AppDatabase>) -> Result<(), Box<dyn std::error::Error>> {
    Startup::new(database).run().await
}