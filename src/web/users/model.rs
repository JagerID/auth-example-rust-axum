use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Debug, ToSchema, Serialize, FromRow)]
pub struct User {
    pub id: uuid::Uuid,

    pub email: String,

    pub name: String,

    pub password: String,

    pub role: String,

    #[serde(rename = "isBlocked")]
    pub is_blocked: bool,

    pub photo: Option<String>,

    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}
