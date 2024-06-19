use scylla::{FromRow, SerializeCql};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema, SerializeCql)]
pub struct CreateUserDto {
    pub email: String,
    pub name: String
}