use std::collections::HashMap;

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
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
        let (status_code, message, validation_errors) = match self {
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
            Json(json!({ "message": message, "validation_errors": validation_errors })),
        )
            .into_response()
    }
}

pub async fn handle_404() -> impl IntoResponse {
    ApiError::NotFound
}

pub fn convert_validation_errors_to_json(
    validation_errors: ValidationErrors,
) -> HashMap<String, Vec<String>> {
    let mut errors = HashMap::new();

    for (key, validation_error_values) in validation_errors.field_errors().into_iter() {
        let error_key_messages: Vec<String> = validation_error_values
            .iter()
            .filter(|error_value| error_value.message.is_some())
            .map(|error_value| match &error_value.message {
                Some(message) => message.to_string(),
                None => "".to_string(),
            })
            .collect();

        errors.insert(key.to_owned(), error_key_messages);
    }

    errors
}
