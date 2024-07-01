use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateProjectDto {
    #[validate(length(min = 1, message = "Name required"))]
    pub name: String,

    #[serde(rename = "isPublic")]
    pub is_public: bool,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateProjectDto {
    #[validate(length(min = 1, message = "Name required"))]
    pub name: Option<String>,

    #[serde(rename = "isPublic")]
    pub is_public: Option<bool>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct FilteredProject {
    pub id: uuid::Uuid,

    pub name: String,

    #[serde(rename = "isPublic")]
    pub is_public: bool,

    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}
