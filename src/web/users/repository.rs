use scylla::Session;

use crate::web::error::ApiError;

use super::{dto::CreateUserDto, model::User};
use tracing::{error, info};

pub async fn get_user_by_id(db: &Session, id: uuid::Uuid) -> Result<User, ApiError> {
    let result = db
        .query("SELECT * FROM idk.users WHERE id = ?", (id,))
        .await
        .map_err(|e| {
            error!("{}", e);
            ApiError::InternalServerError
        })?;

    let iter = result.rows_typed::<User>().map_err(|e| {
        error!("{}", e);
        ApiError::InternalServerError
    })?;

    if let Some(user) = iter.last() {
        Ok(user.unwrap())
    } else {
        Err(ApiError::NotFound)
    }
}

pub async fn create_user(db: &Session, body: CreateUserDto) -> Result<User, ApiError> {
    db.query(
        "INSERT INTO idk.users (id, name, email) VALUES (uuid(), ?, ?)",
        (body.name, body.email.to_string()),
    )
    .await
    .map_err(|e| {
        error!("{}", e);
        ApiError::InternalServerError
    })?;

    let result = db
        .query("SELECT * FROM idk.users WHERE email = ?", (body.email,))
        .await
        .map_err(|e| {
            error!("{}", e);
            ApiError::InternalServerError
        })?;

    let user = result.first_row_typed::<User>().map_err(|e| {
        error!("{}", e);
        ApiError::InternalServerError
    })?;

    Ok(user)
}
