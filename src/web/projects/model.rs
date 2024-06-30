use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, FromRow)]
pub struct Project {
    pub id: uuid::Uuid,

    pub name: String,

    #[serde(rename = "isPublic")]
    pub is_public: bool,

    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,

    pub user_id: uuid::Uuid,
}
