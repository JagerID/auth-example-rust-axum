use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use validator::Validate;

use crate::{
    state::AppState,
    web::{error::ApiError, extractors::UserID},
};

use super::{
    dto::{CreateProjectDto, FilteredProject},
    service,
    utils::{filter_project, filter_projects},
};

const API_TAG: &str = "Projects";

#[utoipa::path(
    post,
    path = "/api/projects",
    tag = API_TAG,

    request_body = CreateProjectDto,

    responses(
        (status = 201, description = "Project created", body = FilteredProject),
        (status = 500, description = "Internal server error")
    ),

    security(
        ("token" = [])
    )
)]
pub async fn create_project(
    UserID(user_id): UserID,
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateProjectDto>,
) -> Result<Json<FilteredProject>, ApiError> {
    match body.validate() {
        Ok(_) => (),
        Err(_) => return Err(ApiError::ValidationError),
    };

    match service::create_project(&state.db, body, user_id).await {
        Ok(created_project) => Ok(Json(filter_project(&created_project))),
        Err(error) => Err(error),
    }
}

#[utoipa::path(
    get,
    path = "/api/projects",
    tag = API_TAG,

    responses(
        (status = 200, description = "Successfully getting projects", body = [FilteredProject])
    ),

    security(
        ("token" = [])
    )
)]
pub async fn get_projects(
    UserID(user_id): UserID,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<FilteredProject>>, ApiError> {
    match service::get_projects(&state.db, user_id).await {
        Ok(projects) => Ok(Json(filter_projects(&projects))),
        Err(error) => Err(error),
    }
}

#[utoipa::path(
    get,
    path = "/api/projects/{id}",
    tag = API_TAG,

    responses(
        (status = 200, description = "Successfully getting project", body = FilteredProject)
    ),

    security(
        ("token" = [])
    )
)]
pub async fn get_project_by_id(
    UserID(user_id): UserID,
    State(state): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<FilteredProject>, ApiError> {
    match service::get_project_by_id(&state.db, id, user_id).await {
        Ok(project) => Ok(Json(filter_project(&project))),
        Err(error) => Err(error),
    }
}

#[utoipa::path(
    delete,
    path = "/api/projects/{id}",
    tag = API_TAG,

    responses(
        (status = 200, description = "Successfully delete project")
    ),

    security(
        ("token" = [])
    )
)]
pub async fn delete_project(
    UserID(user_id): UserID,
    State(state): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<(), ApiError> {
    match service::delete_project(&state.db, id, user_id).await {
        Ok(_) => Ok(()),
        Err(error) => Err(error),
    }
}
