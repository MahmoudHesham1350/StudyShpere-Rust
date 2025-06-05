use axum::{
    extract::{FromRequestParts, Request},
    http::request::Parts,
    middleware::Next,
    response::Response,
    extract::State,
};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::user::User,
    utils::JwtManager,
};

/// Authentication middleware that extracts and validates JWT tokens
pub async fn auth_middleware(
    State(pool): State<Pool<Postgres>>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {

    let headers = request.headers();
    let auth_header = headers.get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(AppError::Unauthorized)?;

    let jwt_secret = std::env::var("JWT_SECRET")
        .map_err(|_| AppError::ValidationError("JWT_SECRET not configured".to_string()))?;
    
    let jwt_manager = JwtManager::new(&jwt_secret);
    
    // Verify the token
    let claims = jwt_manager
        .verify_access_token(auth_header)
        .map_err(|_| AppError::Unauthorized)?;

    let user_id = claims
        .user_id()
        .map_err(|_| AppError::Unauthorized)?;

    // Verify user exists in database
    let _user = User::find_by_id(&pool, user_id)
        .await?
        .ok_or(AppError::Unauthorized)?;

    // Add user_id to request extensions so handlers can access it
    request.extensions_mut().insert(AuthenticatedUser { id: user_id });

    Ok(next.run(request).await)
}

/// Extractor for authenticated user ID from request
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: Uuid,
}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<AuthenticatedUser>()
            .cloned()
            .ok_or(AppError::Unauthorized)
    }
}

/// Optional authentication middleware for endpoints that can work with or without auth
pub async fn optional_auth_middleware(
    mut request: Request,
    next: Next,
) -> Response {
    let pool = request.extensions().get::<Pool<Postgres>>().cloned();
    
    if let Some(pool) = pool {
        let headers = request.headers();
        if let Some(auth_header) = headers.get("authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
        {
            if let Ok(jwt_secret) = std::env::var("JWT_SECRET") {
                let jwt_manager = JwtManager::new(&jwt_secret);
                
                if let Ok(claims) = jwt_manager.verify_access_token(auth_header) {
                    if let Ok(user_id) = claims.user_id() {
                        if let Ok(Some(_)) = User::find_by_id(&pool, user_id).await {
                            request.extensions_mut().insert(AuthenticatedUser { id: user_id });
                        }
                    }
                }
            }
        }
    }
    next.run(request).await
}

/// Extractor for optional authenticated user
#[derive(Debug, Clone)]
pub struct OptionalAuthenticatedUser(pub Option<Uuid>);

impl<S> FromRequestParts<S> for OptionalAuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let user_id = parts
            .extensions
            .get::<AuthenticatedUser>()
            .map(|user| user.id);
        
        Ok(OptionalAuthenticatedUser(user_id))
    }
}