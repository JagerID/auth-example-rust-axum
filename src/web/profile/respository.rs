use sqlx::postgres::PgQueryResult;

use crate::{
    db::postgres::PostgresPool,
    web::{error::ApiError, users::model::User},
};

pub async fn upload_photo(
    db: &PostgresPool,
    id: uuid::Uuid,
    photo: Option<String>,
) -> Result<PgQueryResult, ApiError> {
    sqlx::query(r#"UPDATE users SET photo = $1 WHERE id = $2"#)
        .bind(photo)
        .bind(id)
        .execute(db)
        .await
        .map_err(|_| ApiError::InternalServerError)
}

pub async fn get_profile(db: &PostgresPool, id: uuid::Uuid) -> Result<User, ApiError> {
    sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_one(db)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => ApiError::UserNotFound,
            _ => ApiError::InternalServerError,
        })
}
