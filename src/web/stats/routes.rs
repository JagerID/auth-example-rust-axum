use std::sync::Arc;

use axum::{routing::get, Router};

use crate::state::AppState;

use super::controller::get_users_stat;

pub fn stats_routes() -> Router<Arc<AppState>> {
    Router::new().route("/users", get(get_users_stat))
}
