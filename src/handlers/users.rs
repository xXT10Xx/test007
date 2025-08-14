use crate::{
    database::Database,
    models::{CreateUserRequest, UpdateUserRequest, UserResponse},
    services::UserService,
    utils::AppError,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize)]
pub struct ListUsersQuery {
    #[serde(default = "default_limit")]
    limit: i64,
    #[serde(default)]
    offset: i64,
}

fn default_limit() -> i64 {
    10
}

pub async fn list_users(
    State(database): State<Database>,
    Query(query): Query<ListUsersQuery>,
) -> Result<Json<Vec<UserResponse>>, AppError> {
    let user_service = UserService::new(database.pool().clone());
    let users = user_service.list_users(query.limit, query.offset).await?;
    let user_responses: Vec<UserResponse> = users.into_iter().map(Into::into).collect();

    Ok(Json(user_responses))
}

pub async fn get_user(
    State(database): State<Database>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserResponse>, AppError> {
    let user_service = UserService::new(database.pool().clone());
    let user = user_service
        .get_user_by_id(id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(Json(user.into()))
}

pub async fn create_user(
    State(database): State<Database>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    payload.validate()?;

    let user_service = UserService::new(database.pool().clone());

    if user_service
        .get_user_by_email(&payload.email)
        .await?
        .is_some()
    {
        return Err(AppError::Conflict("Email already exists".to_string()));
    }

    let user = user_service.create_user(payload).await?;

    Ok(Json(user.into()))
}

pub async fn update_user(
    State(database): State<Database>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    payload.validate()?;

    let user_service = UserService::new(database.pool().clone());
    let user = user_service
        .update_user(id, payload)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(Json(user.into()))
}

pub async fn delete_user(
    State(database): State<Database>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let user_service = UserService::new(database.pool().clone());
    let deleted = user_service.delete_user(id).await?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound("User not found".to_string()))
    }
}