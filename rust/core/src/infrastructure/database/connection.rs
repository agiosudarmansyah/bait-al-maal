use turso::{ Builder, Connection, Database };

pub struct AppDatabase {
    db: Database
}

/// Need to be revised...
impl AppDatabase {
    pub async fn new() -> std::result::Result<Self, turso::Error> {
        let db = Builder::new_local("sqlite.db")
        .build()
        .await?;

        Ok(Self { db })
    }

    pub fn connection(&self) -> std::result::Result<Connection, turso::Error> {
        Ok(self.db.connect()?)
    }
}
