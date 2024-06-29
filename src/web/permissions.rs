use std::sync::Arc;

use axum::{async_trait, extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

use crate::state::AppState;

use super::{auth::jwt::decode_token, error::ApiError};

#[derive(Debug)]
pub struct IsAdmin;

#[async_trait]
impl FromRequestParts<Arc<AppState>> for IsAdmin
where
    Arc<AppState>: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let bearer: TypedHeader<Authorization<Bearer>> = parts
            .extract()
            .await
            .map_err(|_| Self::Rejection::Unauthorized)?;

        let token_claims = decode_token(bearer.token(), &state.env.jwt_secret)
            .map_err(|_| ApiError::Unauthorized)?;

        match &token_claims.role[..] {
            "ADMIN" => Ok(Self),
            _ => Err(ApiError::Forbidden),
        }
    }
}
