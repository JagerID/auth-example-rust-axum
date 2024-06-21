use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub enum ApiError {
    // ---------------- Common Errors ----------------
    InternalServerError,
    NotFound,

    // ---------------- Users Errors ----------------
    UserNotFound,
    UserAlreadyExists
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, message) = match self {
            Self::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            Self::NotFound => (StatusCode::NOT_FOUND, "Not found"),
            Self::UserNotFound => (StatusCode::NOT_FOUND, "User not found"),
            Self::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists")
        };

        (status_code, Json(json!({ "message": message }))).into_response()
    }
}
