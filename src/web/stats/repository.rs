use crate::{db::postgres::PostgresPool, web::error::ApiError};

pub async fn get_users_count(db: &PostgresPool) -> Result<i64, ApiError> {
    sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(db)
        .await
        .map_err(|_| ApiError::InternalServerError)
}

pub async fn get_blocked_users_count(db: &PostgresPool) -> Result<i64, ApiError> {
    sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE is_blocked IS TRUE")
        .fetch_one(db)
        .await
        .map_err(|_| ApiError::InternalServerError)
}
