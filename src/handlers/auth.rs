use crate::{
    database::Database,
    models::{AuthResponse, CreateUserRequest, LoginRequest},
    services::{AuthService, UserService},
    utils::AppError,
};
use axum::{extract::State, response::Json};
use validator::Validate;

pub async fn register(
    State(database): State<Database>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    payload.validate()?;

    let user_service = UserService::new(database.pool().clone());
    let auth_service = AuthService::new("your-secret-key".to_string());

    if user_service
        .get_user_by_email(&payload.email)
        .await?
        .is_some()
    {
        return Err(AppError::Conflict("Email already exists".to_string()));
    }

    let user = user_service.create_user(payload).await?;
    let auth_response = auth_service.create_auth_response(user)?;

    Ok(Json(auth_response))
}

pub async fn login(
    State(database): State<Database>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let user_service = UserService::new(database.pool().clone());
    let auth_service = AuthService::new("your-secret-key".to_string());

    let user = user_service
        .verify_password(&payload.email, &payload.password)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?;

    let auth_response = auth_service.create_auth_response(user)?;

    Ok(Json(auth_response))
}