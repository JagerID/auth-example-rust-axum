use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;

use crate::state::AppState;

use super::controller::{get_profile, upload_photo};

pub fn profile_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/upload-photo", post(upload_photo))
        .route("/", get(get_profile))
        .nest_service("/photo", ServeDir::new("public/uploads/profiles"))
}
