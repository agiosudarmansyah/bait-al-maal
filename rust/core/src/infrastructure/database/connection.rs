use turso::{ Builder, Connection, Database };

pub struct AppDatabase {
    db: turso::Database
}

/// Need to be revised...
impl AppDatabase {
    async fn new() -> Result<(), Box<dyn std::error::Error>> {
        let db = Builder::new_local("sqlite.db").build().await?;
        let conn = db.connect()?;

        Ok(())
    }
}
