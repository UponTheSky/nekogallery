use std::time::Duration;

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
};
use sqlx::{pool, sqlite::SqlitePoolOptions, Error, Pool, Sqlite, SqlitePool};

pub async fn setup_pool(
    db_url: &str,
    max_connections: u32,
    timeout: Duration,
) -> Result<Pool<Sqlite>, Error> {
    SqlitePoolOptions::new()
        .max_connections(max_connections)
        .acquire_timeout(timeout)
        .connect(db_url)
        .await
}

pub struct Connection(pub pool::PoolConnection<sqlx::Sqlite>);

#[async_trait]
impl<S> FromRequestParts<S> for Connection
where
    SqlitePool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = SqlitePool::from_ref(state);

        let conn = pool
            .acquire()
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

        Ok(Self(conn))
    }
}
