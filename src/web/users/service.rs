use scylla::Session;

use crate::web::error::ApiError;

use super::{dto::CreateUserDto, model::User, repository};

pub async fn get_user_by_id(db: &Session, id: uuid::Uuid) -> Result<User, ApiError> {
    repository::get_user_by_id(db, id).await
}

pub async fn create_user(db: &Session, body: CreateUserDto) -> Result<User, ApiError> {
    repository::create_user(db, body).await
}
