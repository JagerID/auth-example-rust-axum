use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub enum ApiError {
    // ---------------- Server Errors ----------------
    InternalServerError,
    NotFound,

    // ---------------- Commkon Api Errors ----------------
    ValidationError,
    Forbidden,
    BodyParsingError(String),
    Conflict(String),

    // ---------------- Users Errors ----------------
    UserNotFound,
    UserAlreadyExists,
    Unauthorized,
    InvalidCredentials,
    Blocked,

    // ---------------- Projects Errors ----------------
    ProjectNotFound,

}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, message) = match self {
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_owned(),
            ),
            Self::NotFound => (StatusCode::NOT_FOUND, "Not found".to_string()),
            Self::UserNotFound => (StatusCode::NOT_FOUND, "User not found".to_owned()),
            Self::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists".to_owned()),
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_owned()),
            Self::ValidationError => (StatusCode::BAD_REQUEST, "Validation error".to_owned()),
            Self::InvalidCredentials => {
                (StatusCode::UNAUTHORIZED, "Invalid credentials".to_owned())
            }

            Self::Forbidden => (StatusCode::FORBIDDEN, "Forbidden".to_owned()),
            Self::Blocked => (StatusCode::FORBIDDEN, "Account blocked".to_owned()),

            Self::ProjectNotFound => (StatusCode::NOT_FOUND, "Project not found".to_owned()),
            
            Self::Conflict(string) => (StatusCode::CONFLICT, string),
            Self::BodyParsingError(string) => (StatusCode::BAD_REQUEST, string),
        };

        (status_code, Json(json!({ "message": message }))).into_response()
    }
}

pub async fn handle_404() -> impl IntoResponse {
    ApiError::NotFound
}
