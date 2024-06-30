use tracing::info;

use crate::{db::postgres::PostgresPool, web::error::ApiError};

use super::{
    dto::{ProjectsStat, UsersStat},
    repository,
};

pub async fn get_users_stat(db: &PostgresPool) -> Result<UsersStat, ApiError> {
    let count = repository::get_users_count(db).await?;
    let blocked = repository::get_blocked_users_count(db).await?;

    Ok(UsersStat { count, blocked })
}

pub async fn get_projects_stat(db: &PostgresPool) -> Result<ProjectsStat, ApiError> {
    let count = repository::get_projects_count(db).await?;
    let private = repository::get_private_projects_count(db).await?;

    Ok(ProjectsStat {
        count,
        private,
        public: count - private,
    })
}
