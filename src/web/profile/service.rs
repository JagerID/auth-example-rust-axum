use std::sync::Arc;

use axum_extra::extract::Multipart;
use tokio::{fs::File, io::AsyncWriteExt};
use tracing::info;

use crate::{state::AppState, web::error::ApiError};

use super::respository;

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
            let content_type = &field.content_type().unwrap().to_string();
            let data = field
                .bytes()
                .await
                .map_err(|_| ApiError::InternalServerError)?;

            let path = format!(
                "{}{}{}.{}",
                &state.env.media_path,
                "/profiles/",
                user_id,
                content_type.split("/").last().unwrap().to_string()
            );

            let mut file = File::create(path.to_string())
                .await
                .map_err(|_| ApiError::InternalServerError)?;

            file.write(&data)
                .await
                .map_err(|_| ApiError::InternalServerError)?;

            respository::upload_photo(&state.db, user_id, Some(path)).await?;
        }
    }

    Ok(())
}
