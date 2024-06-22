use std::sync::Arc;

use axum::{routing::post, Router};

use crate::state::AppState;

use super::controller::upload_photo;

pub fn profile_routes() -> Router<Arc<AppState>> {
    Router::new().route("/photo", post(upload_photo))
}
