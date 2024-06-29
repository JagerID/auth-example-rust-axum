use std::sync::Arc;

use axum::{routing::patch, Router};

use crate::state::AppState;

use super::controller::{get_user_by_id, get_users, update_user};
use axum::routing::get;

pub fn users_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_users))
        // .route_layer(middleware::from_extractor_with_state::<
        //     IsAdmin,
        //     Arc<AppState>,
        // >(state))
        .route("/:id", get(get_user_by_id))
        .route("/:id", patch(update_user))
}
