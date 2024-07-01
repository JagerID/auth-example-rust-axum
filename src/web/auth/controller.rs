use std::sync::Arc;

use axum::{extract::State, Json};
use validator::Validate;

use crate::{
    state::AppState,
    web::{
        error::ApiError,
        users::{
            dto::{CreateUserDto, FilteredUser},
            utils::filter_user,
        },
    },
};

use super::{
    dto::{LoginUserDto, RefreshDto, TokensDto},
    service,
};

const API_TAG: &str = "Auth";

#[utoipa::path(
    post,
    path = "/api/auth/register",
    tag = API_TAG,

    request_body = CreateUserDto,

    responses(
        (status = 201, description = "User registered", body = FilteredUser),
        (status = 500, description = "Internal server error")
    ),
)]
pub async fn register_user(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateUserDto>,
) -> Result<Json<FilteredUser>, ApiError> {
    match body.validate() {
        Ok(_) => (),
        Err(error) => return Err(ApiError::ValidationError(error)),
    };

    match service::register_user(&state.db, body).await {
        Ok(created_user) => Ok(Json(filter_user(&created_user))),
        Err(error) => Err(error),
    }
}

#[utoipa::path(
    post,
    path = "/api/auth/login",
    tag = API_TAG,

    request_body = LoginUserDto,

    responses(
        (status = 201, description = "User logged", body = TokensDto),
        (status = 500, description = "Internal server error")
    ),
)]
pub async fn login_user(
    State(state): State<Arc<AppState>>,
    Json(body): Json<LoginUserDto>,
) -> Result<Json<TokensDto>, ApiError> {
    match body.validate() {
        Ok(_) => (),
        Err(errors) => return Err(ApiError::ValidationError(errors)),
    };

    match service::login_user(&state, body).await {
        Ok(tokens) => Ok(Json(tokens)),
        Err(error) => Err(error),
    }
}

#[utoipa::path(
    post,
    path = "/api/auth/refresh",
    tag = API_TAG,

    request_body = RefreshDto,

    responses(
        (status = 201, description = "Tokens refreshed", body = TokensDto),
        (status = 500, description = "Internal server error")
    ),
)]
pub async fn refresh_tokens(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RefreshDto>,
) -> Result<Json<TokensDto>, ApiError> {
    match body.validate() {
        Ok(_) => (),
        Err(errors) => return Err(ApiError::ValidationError(errors)),
    };

    match service::refresh_tokens(&state, body).await {
        Ok(tokens) => Ok(Json(tokens)),
        Err(error) => Err(error),
    }
}
