use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TokenClaims {
    pub sub: uuid::Uuid,
    pub role: String,
    pub is_blocked: bool,
    
    pub iat: usize,
    pub exp: usize,
}
