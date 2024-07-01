use std::collections::HashMap;

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use tracing::info;
use validator::ValidationErrors;

pub enum ApiError {
    // ---------------- Server Errors ----------------
    InternalServerError,
    NotFound,

    // ---------------- Commkon Api Errors ----------------
    ValidationError(ValidationErrors),
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
        let (status_code, message, details) = match self {
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_owned(),
                None,
            ),
            Self::NotFound => (StatusCode::NOT_FOUND, "Not found".to_owned(), None),
            Self::UserNotFound => (StatusCode::NOT_FOUND, "User not found".to_owned(), None),
            Self::UserAlreadyExists => {
                (StatusCode::CONFLICT, "User already exists".to_owned(), None)
            }
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_owned(), None),
            Self::ValidationError(errors) => (
                StatusCode::BAD_REQUEST,
                "Validation error".to_owned(),
                Some(convert_validation_errors_to_json(errors)),
            ),
            Self::InvalidCredentials => (
                StatusCode::UNAUTHORIZED,
                "Invalid credentials".to_owned(),
                None,
            ),

            Self::Forbidden => (StatusCode::FORBIDDEN, "Forbidden".to_owned(), None),
            Self::Blocked => (StatusCode::FORBIDDEN, "Account blocked".to_owned(), None),

            Self::ProjectNotFound => (StatusCode::NOT_FOUND, "Project not found".to_owned(), None),

            Self::Conflict(string) => (StatusCode::CONFLICT, string, None),
            Self::BodyParsingError(string) => (StatusCode::BAD_REQUEST, string, None),
        };

        (
            status_code,
            Json(json!({ "message": message, "details": details })),
        )
            .into_response()
    }
}

pub async fn handle_404() -> impl IntoResponse {
    ApiError::NotFound
}

pub fn convert_validation_errors_to_json(validation_errors: ValidationErrors) {
    // let mut errors = HashMap::new();

    for (key, value) in validation_errors.field_errors().into_iter() {
        info!("{} / {:#?}", key, value)
    }
}
