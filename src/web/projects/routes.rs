use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::state::AppState;

use super::controller::{create_project, get_project_by_id, get_projects};

pub fn projects_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_project).get(get_projects))
        .route("/:id", get(get_project_by_id))
}
// e2790f77-dc98-4a91-b5b7-0a1961fc0ae1
