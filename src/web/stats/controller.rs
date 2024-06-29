use std::sync::Arc;

use axum::{extract::State, Json};

use crate::{state::AppState, web::error::ApiError};

use super::{dto::UsersStat, service};

const API_TAG: &str = "Stats";

#[utoipa::path(
    get,
    path = "/api/stats/users",
    tag = API_TAG,

    security(
        ("token" = [])
    )
)]
pub async fn get_users_stat(
    State(state): State<Arc<AppState>>,
) -> Result<Json<UsersStat>, ApiError> {
    match service::get_users_stat(&state.db).await {
        Ok(stats) => Ok(Json(stats)),
        Err(error) => Err(error),
    }
}
