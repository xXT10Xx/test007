use crate::{database::Database, services::AuthService, utils::AppError};
use axum::{
    extract::{Request, State},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};

pub async fn auth_middleware(
    State(_database): State<Database>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let path = request.uri().path();
    
    if path == "/health" || path.starts_with("/api/auth/") {
        return Ok(next.run(request).await);
    }

    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or_else(|| AppError::Unauthorized("Missing authorization header".to_string()))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| AppError::Unauthorized("Invalid authorization header format".to_string()))?;

    let auth_service = AuthService::new("your-secret-key".to_string());
    let claims = auth_service
        .verify_token(token)
        .map_err(|_| AppError::Unauthorized("Invalid token".to_string()))?;

    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}