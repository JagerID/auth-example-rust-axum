use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSchema, FromRow, Validate)]
pub struct CreateUserDto {
    #[validate(email(message = "Incorrect email format"))]
    pub email: String,

    #[validate(length(min = 1, message = "Name required"))]
    pub name: String,

    #[validate(length(min = 4, message = "Password length must be greater than 4"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, FromRow)]
pub struct FilteredUser {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub role: String,
    pub photo: Option<String>,

    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}
