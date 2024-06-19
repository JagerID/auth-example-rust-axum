use std::sync::Arc;

use axum::{extract::{Path, State}, Json};

use crate::{state::AppState, web::error::ApiError};

use super::{dto::CreateUserDto, model::User, service};

const API_TAG: &str = "Users";

#[utoipa::path(
    get,
    path = "/api/v1/users/{id}",
    tag = API_TAG,
    
    params(
        ("id" = uuid::Uuid, Path, description = "User id")
    )
)]
pub async fn get_user_by_id(State(state): State<Arc<AppState>>, Path(id): Path<uuid::Uuid>) -> Result<Json<User>, ApiError> {
    match service::get_user_by_id(&state.db, id).await {
        Ok(user) => Ok(Json(user)),
        Err(err) => Err(err)
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/users",
    tag = API_TAG,

    request_body = CreateUserDto
)]
pub async fn create_user(State(state): State<Arc<AppState>>, Json(body): Json<CreateUserDto>) -> Result<Json<User>, ApiError> {
    match service::create_user(&state.db, body).await {
        Ok(user) => Ok(Json(user)),
        Err(err) => Err(err)
    }
}