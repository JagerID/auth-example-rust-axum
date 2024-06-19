use std::sync::Arc;

use axum::{routing::post, Router};

use crate::state::AppState;

use super::controller::{create_user, get_user_by_id};
use axum::routing::get;

pub fn users_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/:id", get(get_user_by_id))
        .route("/", post(create_user))
}
