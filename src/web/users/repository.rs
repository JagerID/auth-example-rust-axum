use sqlx::postgres::PgQueryResult;

use crate::{db::postgres::PostgresPool, web::error::ApiError};

use super::{
    dto::{CreateUserDto, UpdateUserDto},
    model::User,
};

pub async fn create_user(db: &PostgresPool, body: CreateUserDto) -> Result<User, ApiError> {
    sqlx::query_as!(
        User,
        r#"INSERT INTO users (email, name, password, role) VALUES ($1, $2, $3, $4) RETURNING *"#,
        body.email.to_owned(),
        body.name.to_owned(),
        body.password.to_owned(),
        "USER"
    )
    .fetch_one(db)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(error) => {
            if error.code().unwrap() == "23505" {
                ApiError::UserAlreadyExists
            } else {
                ApiError::InternalServerError
            }
        }
        _ => ApiError::InternalServerError,
    })
}

pub async fn get_users(db: &PostgresPool) -> Result<Vec<User>, ApiError> {
    sqlx::query_as("SELECT * FROM users")
        .fetch_all(db)
        .await
        .map_err(|_| ApiError::InternalServerError)
}

pub async fn get_user_by_id(db: &PostgresPool, id: uuid::Uuid) -> Result<User, ApiError> {
    sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_one(db)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => ApiError::UserNotFound,
            _ => ApiError::InternalServerError,
        })
}

pub async fn get_user_by_email(db: &PostgresPool, email: String) -> Result<User, ApiError> {
    sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
        .fetch_one(db)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => ApiError::UserNotFound,
            _ => ApiError::InternalServerError,
        })
}

pub async fn update_user(
    db: &PostgresPool,
    id: uuid::Uuid,
    body: UpdateUserDto,
) -> Result<PgQueryResult, ApiError> {
    sqlx::query(r#"UPDATE users SET name = $1 WHERE id = $2"#)
        .bind(body.name)
        .bind(id)
        .execute(db)
        .await
        .map_err(|_| ApiError::InternalServerError)
}

pub async fn delete_user(db: &PostgresPool, id: uuid::Uuid) -> Result<PgQueryResult, ApiError> {
    sqlx::query(r#"DELETE FROM users WHERE id = $1"#)
        .bind(id)
        .execute(db)
        .await
        .map_err(|_| ApiError::InternalServerError)
}
