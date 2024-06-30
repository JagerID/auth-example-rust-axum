use crate::{db::postgres::PostgresPool, web::error::ApiError};

// ---------------------- USERS ----------------------

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

// ---------------------- PROJECTS ----------------------

pub async fn get_projects_count(db: &PostgresPool) -> Result<i64, ApiError> {
    sqlx::query_scalar("SELECT COUNT(*) FROM projects")
        .fetch_one(db)
        .await
        .map_err(|_| ApiError::InternalServerError)
}

pub async fn get_private_projects_count(db: &PostgresPool) -> Result<i64, ApiError> {
    sqlx::query_scalar("SELECT COUNT(*) FROM projects WHERE is_public IS FALSE")
        .fetch_one(db)
        .await
        .map_err(|_| ApiError::InternalServerError)
}
