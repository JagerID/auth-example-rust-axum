use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub enum ApiError {
    // ---------------- Server Errors ----------------
    InternalServerError,

    // ---------------- Commkon Api Errors ----------------
    NotFound,
    BodyParsingError(String),

    // ---------------- Users Errors ----------------
    UserNotFound,
    UserAlreadyExists,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, message) = match self {
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
            Self::NotFound => (StatusCode::NOT_FOUND, "Not found".to_string()),
            Self::UserNotFound => (StatusCode::NOT_FOUND, "User not found".to_string()),
            Self::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists".to_string()),
            Self::BodyParsingError(string) => (StatusCode::BAD_REQUEST, string),
        };

        (status_code, Json(json!({ "message": message }))).into_response()
    }
}
