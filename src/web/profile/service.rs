use std::sync::Arc;

use axum_extra::extract::Multipart;
use tokio::{fs::File, io::AsyncWriteExt};
use tracing::{error, info};

use crate::{
    db::postgres::PostgresPool,
    state::AppState,
    web::{error::ApiError, users::model::User},
};

use super::respository;

const ACCEPTED_PHOTO_CONTENT_TYPES: &'static [&'static str] = &[content_types::JPEG, content_types::PNG, content_types::WEBP];

pub async fn upload_photo(
    state: &Arc<AppState>,
    user_id: uuid::Uuid,
    mut multipart: Multipart,
) -> Result<(), ApiError> {
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        info!("{}", e);
        ApiError::InternalServerError
    })? {
        let name = field.name().unwrap().to_string();

        if &name == "photo" {
            let file_content_type = &field.content_type().unwrap();

            if !ACCEPTED_PHOTO_CONTENT_TYPES.contains(file_content_type) {
                return Err(ApiError::BodyParsingError("Invalid file type".to_owned()))
            }

            let file_type = &field
                .file_name()
                .unwrap()
                .split(".")
                .last()
                .unwrap()
                .to_string();
            let data = field.bytes().await.map_err(|e| {
                error!("{}", e);
                ApiError::InternalServerError
            })?;

            let file_name = format!("{}.{}", user_id, file_type);

            let path = format!("{}{}{}", &state.env.media_path, "/profiles/", file_name);

            let mut file = File::create(path.to_string()).await.map_err(|e| {
                error!("{}", e);
                ApiError::InternalServerError
            })?;

            file.write(&data).await.map_err(|e| {
                error!("{}", e);
                ApiError::InternalServerError
            })?;

            respository::upload_photo(&state.db, user_id, Some(file_name)).await?;
        } else {
            return Err(ApiError::BodyParsingError(
                "Use `photo` field name".to_owned(),
            ));
        }
    }

    Ok(())
}

pub async fn get_profile(db: &PostgresPool, user_id: uuid::Uuid) -> Result<User, ApiError> {
    respository::get_profile(db, user_id).await
}
