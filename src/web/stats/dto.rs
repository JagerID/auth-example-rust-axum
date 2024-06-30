use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct UsersStat {
    pub count: i64,
    pub blocked: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ProjectsStat {
    pub count: i64,
    pub private: i64,
    pub public: i64,
}
