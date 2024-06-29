use std::sync::Arc;

use axum::extract::State;
use axum_extra::extract::Multipart;

use crate::{
    state::AppState,
    web::{error::ApiError, extractors::UserID},
};

use super::service;

const API_TAG: &str = "Profile";

#[utoipa::path(
    post,
    path = "/api/profile/upload-photo",
    tag = API_TAG,

    request_body(content_type = "multipart/formdata", content = Multipart),

    security(
        ("token" = [])
    )
)]
pub async fn upload_photo(
    State(state): State<Arc<AppState>>,
    UserID(user_id): UserID,
    multipart: Multipart,
) -> Result<(), ApiError> {
    service::upload_photo(&state, user_id, multipart).await
}
