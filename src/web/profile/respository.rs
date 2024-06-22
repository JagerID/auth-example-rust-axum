use sqlx::postgres::PgQueryResult;

use crate::{db::postgres::PostgresPool, web::error::ApiError};

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
