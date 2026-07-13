use sqlx::SqlitePool;

use crate::shared::error::Result;

pub struct Database {
    pub pool: SqlitePool
}

impl Database {
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    async fn connect_with(url: &str) -> Result<Self> {
        let pool = SqlitePool::connect(url).await?;

        Ok(Self::new(pool))
    }

    pub async fn connect() -> Result<Self> {
        let url = std::env::var("DATABASE_URL")?;
        Self::connect_with(&url).await
    }

    pub async fn memory() -> Result<Self> {
        let db = Self::connect_with("sqlite::memory:?cache=shared").await?;

        sqlx::migrate!()
            .run(db.pool())
            .await?;

        Ok(db)
    }

    pub async fn test() -> Result<Self> {
        let url = std::env::var("DATABASE_TEST_URL")?;
        Self::connect_with(&url).await
    }
}
