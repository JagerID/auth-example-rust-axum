use axum::{body::Body, extract::Request, http::header, middleware::Next, response::IntoResponse};
use tracing::info;

use super::error::ApiError;

pub async fn auth_guard(req: Request<Body>, next: Next) -> Result<impl IntoResponse, ApiError> {
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
    // let token = req
    //     .headers()
    //     .get(header::AUTHORIZATION)
    //     .and_then(|auth_header| auth_header.to_str().ok())
    //     .and_then(|auth_value| {
    //         if auth_value.starts_with("Bearer ") {
    //             Some(auth_value[7..].to_owned())
    //         } else {
    //             None
    //         }
    //     })
    //     .ok_or_else(|| ApiError::Unauthorized)?;

    // info!("token: {}", token);

    // decode::<TokenClaims>(
    //     &token,
    //     &DecodingKey::from_secret("abdsf".as_bytes()),
    //     &Validation::default(),
    // )
    // .map_err(|_| ApiError::Unauthorized)?
    // .claims;

    info!("Authorized: {}", token);

    Ok(next.run(req).await)
}
