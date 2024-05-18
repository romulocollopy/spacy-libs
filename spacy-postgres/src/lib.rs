pub mod errors;
pub use errors::SpacyDBError;
use std::sync::Arc;
use uuid::Uuid;

use sqlx::{
    pool::PoolConnection,
    postgres::{PgPoolOptions, PgRow},
    FromRow, PgPool
};

pub struct DB {
    pool: Arc<PgPool>,
}

#[derive(FromRow)]
pub struct Check {
    pub value: i64,
}

#[derive(FromRow, Debug)]
pub struct SKU {
    pub id: Uuid,
    pub quantity: i64,
    pub name: String,
}

impl DB {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn check(&self) -> Result<(), SpacyDBError> {
        let value = 150_i64;
        let check = self
            .fetch_one::<Check>("SELECT $1 as value".to_owned(), value)
            .await?;

        assert_eq!(check.value, value);
        Ok(())
    }

    pub async fn fetch_one<T: for<'a> FromRow<'a, PgRow>>(
        &self,
        query: String,
        binds: i64,
    ) -> Result<T, SpacyDBError> {
        let row = sqlx::query(&query)
            .bind(binds)
            .fetch_one(&*self.pool)
            .await
            .map_err(|e| SpacyDBError::new(format!("{}", e)))?;

        T::from_row(&row).map_err(|e| SpacyDBError::new(format!("{}", e)))
    }

    pub async fn fetch_all<T: for<'a> FromRow<'a, PgRow>>(
        &self,
        query: String,
        binds: i64,
    ) -> Result<Vec<T>, SpacyDBError> {
        let rows = sqlx::query(&query)
            .bind(binds)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| SpacyDBError::new(format!("{}", e)))?;

        rows.iter()
            .map(|r| T::from_row(r).map_err(|e| SpacyDBError::new(format!("{}", e))))
            .collect()
    }

    pub fn get_db_connection(&self) -> Option<PoolConnection<sqlx::Postgres>> {
        self.pool.try_acquire()
    }
}

pub async fn get_db_connection(database_url: String) -> DB {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Could not establish a connection with the database");

    DB::new(Arc::new(pool))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_db_connection() {
        let database_url = "postgres://postgres:pass@db/app".to_owned();
        get_db_connection(database_url).await;
    }

    #[tokio::test]
    #[should_panic]
    async fn test_get_db_connection_panics() {
        let database_url = "postgres://postgres:pass@wrong/app".to_owned();
        get_db_connection(database_url).await;
    }

    #[tokio::test]
    async fn test_fetch_all() {
        let database_url = "postgres://postgres:pass@db/app".to_owned();
        let db = get_db_connection(database_url).await;
        sqlx::migrate!().run(&*db.pool).await.unwrap();

        let skus: Vec<SKU> = db
            .fetch_all("SELECT * from sku WHERE quantity > $1".to_owned(), 1)
            .await
            .unwrap();
        assert_eq!(skus.len(), 1);
        assert_eq!(skus[0].quantity, 3)
    }

    #[tokio::test]
    async fn test_get_db_connection_from_pool() {
        let database_url = "postgres://postgres:pass@db/app".to_owned();
        let db = get_db_connection(database_url).await;
        db.get_db_connection();
    }
}
