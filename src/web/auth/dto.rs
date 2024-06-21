use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Validate, Deserialize, ToSchema)]
pub struct LoginUserDto {
    #[validate(email(message = "Incorrect email format"))]
    pub email: String,

    #[validate(length(min = 4, message = "Password length must be greater than 4"))]
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TokensDto {
    pub token: String,
    pub refresh: String,
}

#[derive(Debug, Validate, Deserialize, ToSchema)]
pub struct RefreshDto {
    pub refresh: String,
}
