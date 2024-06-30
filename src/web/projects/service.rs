use crate::{db::postgres::PostgresPool, web::error::ApiError};

use super::{dto::CreateProjectDto, model::Project, repository};

pub async fn create_project(
    db: &PostgresPool,
    body: CreateProjectDto,
    user_id: uuid::Uuid,
) -> Result<Project, ApiError> {
    repository::create_project(db, body, user_id).await
}

pub async fn get_project_by_id(
    db: &PostgresPool,
    id: uuid::Uuid,
    user_id: uuid::Uuid,
) -> Result<Project, ApiError> {
    let project = repository::get_project_by_id(db, id).await?;

    if !project.is_public && project.user_id != user_id {
        return Err(ApiError::Forbidden);
    }

    Ok(project)
}

pub async fn get_projects(
    db: &PostgresPool,
    user_id: uuid::Uuid,
) -> Result<Vec<Project>, ApiError> {
    repository::get_projects(db, user_id).await
}

pub async fn delete_project(
    db: &PostgresPool,
    id: uuid::Uuid,
    user_id: uuid::Uuid,
) -> Result<(), ApiError> {
    let project = repository::get_project_by_id(db, id).await?;

    if project.user_id != user_id {
        return Err(ApiError::Forbidden);
    }

    let delete_result = repository::delete_project(db, id).await?;

    if delete_result.rows_affected() == 0 {
        return Err(ApiError::ProjectNotFound);
    }

    Ok(())
}
