use crate::{db::postgres::PostgresPool, web::error::ApiError};

use super::{
    dto::{CreateUserDto, UpdateUserDto},
    model::User,
    repository,
};

pub async fn create_user(db: &PostgresPool, body: CreateUserDto) -> Result<User, ApiError> {
    repository::create_user(db, body).await
}

pub async fn get_users(db: &PostgresPool) -> Result<Vec<User>, ApiError> {
    repository::get_users(db).await
}

pub async fn get_user_by_id(db: &PostgresPool, id: uuid::Uuid) -> Result<User, ApiError> {
    repository::get_user_by_id(db, id).await
}

pub async fn update_user(
    db: &PostgresPool,
    id: uuid::Uuid,
    body: UpdateUserDto,
) -> Result<User, ApiError> {
    let update_result = repository::update_user(db, id, body).await?;

    if update_result.rows_affected() == 0 {
        return Err(ApiError::UserNotFound);
    }

    repository::get_user_by_id(db, id).await
}
