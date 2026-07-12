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
        sqlx::migrate!()
            .run(self.database.pool())
            .await?;

        Ok(())
    }
}

async fn startup(database: Arc<Database>) -> Result<(), Box<dyn std::error::Error>> {
    Startup::new(database).run().await
}