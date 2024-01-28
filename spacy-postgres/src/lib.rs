use std::sync::Arc;

use sqlx::{postgres::PgPoolOptions, PgPool};

pub struct DB {
    pool: Arc<PgPool>,
}

impl DB {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn check(&self) -> Result<(i64,), sqlx::Error> {
        let value = 150_i64;
        let row: (i64,) = sqlx::query_as("SELECT $1")
            .bind(value)
            .fetch_one(&*self.pool)
            .await?;

        assert_eq!(row.0, value);
        Ok(row)
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
}
