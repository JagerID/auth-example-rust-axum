use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Validate, ToSchema, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: uuid::Uuid,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 1))]
    pub name: String,

    #[validate(length(min = 4))]
    pub password: String,

    pub role: String,

    pub is_blocked: bool,

    pub photo: Option<String>,

    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}
