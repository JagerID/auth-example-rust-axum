use std::sync::Arc;

use axum::{middleware, routing::{delete, patch}, Router};

use crate::{state::AppState, web::permissions::IsAdmin};

use super::controller::{delete_user, get_user_by_id, get_users, update_user};
use axum::routing::get;

pub fn users_routes(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_users))
        .route("/:id", delete(delete_user))
        .route_layer(middleware::from_extractor_with_state::<
            IsAdmin,
            Arc<AppState>,
        >(state))
        .route("/:id", get(get_user_by_id))
        .route("/:id", patch(update_user))
}
