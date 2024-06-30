use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct UsersStat {
    pub count: i64,
    pub blocked: i64,
}
