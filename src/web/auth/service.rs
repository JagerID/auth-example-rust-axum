use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use tracing::info;

use crate::{
    db::postgres::PostgresPool,
    state::AppState,
    web::{
        auth::jwt::generate_token,
        error::ApiError,
        users::{self, dto::CreateUserDto, model::User},
    },
};

use super::{
    dto::{LoginUserDto, RefreshDto, TokensDto},
    jwt::decode_token,
    utils::{hash_password, match_passwords},
};

pub async fn register_user(db: &PostgresPool, mut body: CreateUserDto) -> Result<User, ApiError> {
    let hashed_password =
        hash_password(&body.password).map_err(|_| ApiError::InternalServerError)?;

    body.password = hashed_password;

    users::service::create_user(db, body).await
}

pub async fn login_user(state: &AppState, body: LoginUserDto) -> Result<TokensDto, ApiError> {
    let user = users::repository::get_user_by_email(&state.db, body.email.to_string())
        .await
        .map_err(|_| ApiError::InvalidCredentials)?;

    let is_password_match = match_passwords(&user.password, &body.password)
        .map_err(|_| ApiError::InternalServerError)?;

    if !is_password_match {
        return Err(ApiError::InvalidCredentials);
    }

    let token = generate_token(user.id, &state.env.jwt_secret, &state.env.jwt_token_exp)
        .map_err(|_| ApiError::InternalServerError)?;
    let refresh = generate_token(user.id, &state.env.jwt_secret, &state.env.jwt_refresh_exp)
        .map_err(|_| ApiError::InternalServerError)?;

    Ok(TokensDto { token, refresh })
}

pub async fn refresh_tokens(state: &AppState, body: RefreshDto) -> Result<TokensDto, ApiError> {
    let claims = decode_token(&body.refresh, &state.env.jwt_secret)
        .map_err(|_| ApiError::InternalServerError)?;

    let token = generate_token(claims.sub, &state.env.jwt_secret, &state.env.jwt_token_exp)
        .map_err(|_| ApiError::InternalServerError)?;
    let refresh = generate_token(
        claims.sub,
        &state.env.jwt_secret,
        &state.env.jwt_refresh_exp,
    )
    .map_err(|_| ApiError::InternalServerError)?;

    Ok(TokensDto { token, refresh })
}
