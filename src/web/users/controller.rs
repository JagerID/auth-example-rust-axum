use std::sync::Arc;

use axum::{extract::{Path, State}, Json};
use validator::Validate;

use crate::{state::AppState, web::error::ApiError};

use super::{dto::{FilteredUser, UpdateUserDto}, service, utils::{filter_user, filter_users}};

const API_TAG: &str = "Users";

#[utoipa::path(
    get,
    path = "/api/v1/users/{id}",
    tag = API_TAG,
    
    params(
        ("id" = uuid::Uuid, Path, description = "User id")
    ),

    responses(
        (status = 200, description = "Successfully getting user by id", body = FilteredUser),
        (status = 500, description = "Internal server error")
    ),
)]
pub async fn get_user_by_id(
    State(state): State<Arc<AppState>>, Path(id): Path<uuid::Uuid>
) -> Result<Json<FilteredUser>, ApiError> {
    match service::get_user_by_id(&state.db, id).await {
        Ok(user) => Ok(Json(filter_user(&user))),
        Err(err) => Err(err)
    }
}


#[utoipa::path(
    get,
    path = "/api/v1/users",
    tag = API_TAG,

    responses(
        (status = 200, description = "Successfully getting users", body = [FilteredUser]),
        (status = 500, description = "Internal server error")
    ),
)]
pub async fn get_users(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<FilteredUser>>, ApiError> {
    match service::get_users(&state.db).await {
        Ok(users) => Ok(Json(filter_users(&users))),
        Err(error) => Err(error),
    }
}

#[utoipa::path(
    patch,
    path = "/api/v1/users/{id}",
    tag = API_TAG,

    params(
        ("id" = uuid::Uuid, Path, description = "User id")
    ),

    request_body = UpdateUserDto,

    responses(
        (status = 200, description = "Successfully update user", body = FilteredUser),
        (status = 400, description = "bad_request"),
        (status = 404, description = "users.user_not_found"),
        (status = 500, description = "app.internal_server_error")
    ),
)]
pub async fn update_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
    Json(body): Json<UpdateUserDto>,
) -> Result<Json<FilteredUser>, ApiError> {
    match body.validate() {
        Ok(_) => (),
        Err(_) => return Err(ApiError::BodyParsingError("Cannot parse body".to_owned())),
    };

    match service::update_user(&state.db, id, body).await {
        Ok(updated_user) => Ok(Json(filter_user(&updated_user))),
        Err(error) => Err(error),
    }
}


// #[utoipa::path(
//     post,
//     path = "/api/v1/users",
//     tag = API_TAG,

//     request_body = CreateUserDto
// )]
// pub async fn create_user(State(state): State<Arc<AppState>>, Json(body): Json<CreateUserDto>) -> Result<Json<FilteredUser>, ApiError> {
//     match service::create_user(&state.db, body).await {
//         Ok(user) => Ok(Json(filter_user(&user))),
//         Err(err) => Err(err)
//     }
// }