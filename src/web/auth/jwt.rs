use chrono::Duration;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use crate::web::auth::model::TokenClaims;

pub fn generate_token(
    user_id: uuid::Uuid,
    role: &str,
    is_blocked: bool,

    secret: &str,
    exp: &i64,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(*exp)).timestamp() as usize;

    let claims = TokenClaims {
        sub: user_id,
        role: role.to_string(),
        is_blocked,
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;

    Ok(token)
}

pub fn decode_token(token: &str, secret: &str) -> Result<TokenClaims, jsonwebtoken::errors::Error> {
    let token_data = decode::<TokenClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;

    let claims = token_data.claims;

    Ok(claims)
}
