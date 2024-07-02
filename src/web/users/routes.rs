use std::sync::Arc;

use axum::{
    middleware,
    routing::{delete, get},
    Router,
};

use crate::{state::AppState, web::permissions::IsAdmin};

use super::controller::{delete_user, get_user_by_id, get_users, update_user};

pub fn users_routes(state: &Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_users))
        .route("/:id", delete(delete_user))
        .route_layer(middleware::from_extractor_with_state::<
            IsAdmin,
            Arc<AppState>,
        >(state.clone()))
        .route("/:id", get(get_user_by_id).patch(update_user))
}
