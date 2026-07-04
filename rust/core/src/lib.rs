/// Handlers and infrastructures (auth, database, sync) of the app.
pub mod app_state;
pub mod infrastructure;

use std::sync::Arc;

pub use crate::infrastructure::database::AppDatabase;

/// Features
pub mod account;
pub mod analytics;
pub mod budget;
pub mod category;
pub mod user;

pub use crate::budget::{
    Budget,
};
pub use crate::category::Categories;
pub use crate::user::UserCredentials;

pub struct Startup {
    database: Arc<AppDatabase>
}

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