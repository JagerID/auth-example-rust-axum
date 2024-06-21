use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::IntoResponse,
};
use tracing::info;

use crate::{state::AppState, web::auth::jwt::decode_token};

use super::error::ApiError;

pub async fn auth_guard(
    State(state): State<Arc<AppState>>,
    req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, ApiError> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer ") {
                Some(auth_value[7..].to_owned())
            } else {
                None
            }
        })
        .ok_or_else(|| ApiError::Unauthorized)?;

    info!("TOKEN: {}", token);

    decode_token(&token, &state.env.jwt_secret).map_err(|_| ApiError::Unauthorized)?;

    Ok(next.run(req).await)
}
