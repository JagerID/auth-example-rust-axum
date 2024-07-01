use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::state::AppState;

use super::controller::{
    create_project, delete_project, get_project_by_id, get_projects, update_project,
};

pub fn projects_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_project).get(get_projects))
        .route(
            "/:id",
            get(get_project_by_id)
                .delete(delete_project)
                .patch(update_project),
        )
}
