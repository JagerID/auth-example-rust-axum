use std::sync::Arc;

use axum::{routing::post, Router};
use tower_http::services::ServeDir;

use crate::state::AppState;

use super::controller::upload_photo;

pub fn profile_routes() -> Router<Arc<AppState>> {
    Router::new()
    .route("/upload-photo", post(upload_photo))
    .nest_service("/photo", ServeDir::new("public/uploads/profiles"))
}
