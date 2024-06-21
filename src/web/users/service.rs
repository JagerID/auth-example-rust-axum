use crate::{db::postgres::PostgresPool, web::error::ApiError};

use super::{model::User, repository};

pub async fn get_users(db: &PostgresPool) -> Result<Vec<User>, ApiError> {
    repository::get_users(db).await
}

pub async fn get_user_by_id(db: &PostgresPool, id: uuid::Uuid) -> Result<User, ApiError> {
    repository::get_user_by_id(db, id).await
}

