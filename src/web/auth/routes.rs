use std::sync::Arc;

use axum::{routing::post, Router};

use crate::state::AppState;

use super::controller::{login_user, refresh_tokens, register_user};

pub fn auth_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user))
        .route("/refresh", post(refresh_tokens))
}
