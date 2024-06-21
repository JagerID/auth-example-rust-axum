use std::sync::Arc;

use axum::{routing::post, Router};

use crate::state::AppState;

use super::controller::{get_user_by_id, get_users};
use axum::routing::get;

pub fn users_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_users))
        .route("/:id", get(get_user_by_id))
    // .route("/", post(create_user))
}
