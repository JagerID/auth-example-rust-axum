use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UsersStat {
    pub count: i64,
    pub blocked: i64,
}
