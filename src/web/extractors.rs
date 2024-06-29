use std::sync::Arc;

use axum::{async_trait, extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

use crate::{
    state::AppState,
    web::{auth::jwt::decode_token, error::ApiError},
};

pub struct UserID(pub uuid::Uuid);

#[async_trait]
impl FromRequestParts<Arc<AppState>> for UserID
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

        Ok(Self(token_claims.sub))
    }
}
