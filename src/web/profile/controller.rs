use std::sync::Arc;

use axum::{extract::State, Json};
use axum_extra::extract::Multipart;

use crate::{
    state::AppState,
    web::{
        error::ApiError,
        extractors::UserID,
        users::{dto::FilteredUser, utils::filter_user},
    },
};

use super::service;

const API_TAG: &str = "Profile";

#[utoipa::path(
    post,
    path = "/api/profile/upload-photo",
    tag = API_TAG,

    request_body(content_type = "multipart/formdata", content = UploadPhoto),

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

#[utoipa::path(
    get,
    path = "/api/profile",
    tag = API_TAG,

    security(
        ("token" = [])
    )
)]
pub async fn get_profile(
    State(state): State<Arc<AppState>>,
    UserID(user_id): UserID,
) -> Result<Json<FilteredUser>, ApiError> {
    match service::get_profile(&state.db, user_id).await {
        Ok(user) => Ok(Json(filter_user(&user))),
        Err(error) => Err(error),
    }
}
