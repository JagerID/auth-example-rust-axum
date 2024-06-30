use crate::{db::postgres::PostgresPool, web::error::ApiError};

use super::{dto::CreateProjectDto, model::Project};

pub async fn create_project(
    db: &PostgresPool,
    body: CreateProjectDto,
    user_id: uuid::Uuid,
) -> Result<Project, ApiError> {
    sqlx::query_as!(
        Project,
        r#"INSERT INTO projects (name, user_id) VALUES ($1, $2) RETURNING *"#,
        body.name.to_owned(),
        user_id
    )
    .fetch_one(db)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(e) => match e.kind() {
            sqlx::error::ErrorKind::ForeignKeyViolation => ApiError::UserNotFound,
            _ => ApiError::InternalServerError,
        },
        _ => ApiError::InternalServerError,
    })
}

pub async fn get_project_by_id(db: &PostgresPool, id: uuid::Uuid) -> Result<Project, ApiError> {
    sqlx::query_as!(Project, "SELECT * FROM projects WHERE id = $1", id)
        .fetch_one(db)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => ApiError::ProjectNotFound,
            _ => ApiError::InternalServerError,
        })
}

pub async fn get_projects(
    db: &PostgresPool,
    user_id: uuid::Uuid,
) -> Result<Vec<Project>, ApiError> {
    sqlx::query_as("SELECT * FROM projects WHERE user_id = $1")
        .bind(user_id)
        .fetch_all(db)
        .await
        .map_err(|_| ApiError::InternalServerError)
}
