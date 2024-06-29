use crate::{db::postgres::PostgresPool, web::error::ApiError};

use super::{dto::UsersStat, repository};

pub async fn get_users_stat(db: &PostgresPool) -> Result<UsersStat, ApiError> {
    let count = repository::get_users_count(db).await?;
    let blocked = repository::get_blocked_users_count(db).await?;

    Ok(UsersStat { count, blocked })
}
