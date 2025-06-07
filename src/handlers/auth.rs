use axum::{
    extract::{State, Json},
    http::StatusCode,
};
use sqlx::{Pool, Postgres};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::{
    errors::AppError,
    models::user::{User, NewUser},
    utils::{JwtManager, PasswordUtils, EmailUtils, UsernameUtils},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub user: UserResponse,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: usize, // in seconds
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            email: user.email,
            username: user.username,
            created_at: user.created_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: usize,
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub message: String,
}

pub async fn register_handler(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), AppError> {
    // Validate input
    println!("Validating registration input...");
    validate_registration_input(&payload)?;

    // Check if user already exists
    if User::find_by_email(&pool, &payload.email).await?.is_some() {
        return Err(AppError::ValidationError("Email already registered".to_string()));
    }

    if User::find_by_username(&pool, &payload.username).await?.is_some() {
        return Err(AppError::ValidationError("Username already taken".to_string()));
    }

    // Hash password
    let password_hash = PasswordUtils::hash_password(&payload.password)
        .map_err(|e| AppError::ValidationError(format!("Password hashing failed: {}", e)))?;

    // Create new user
    let new_user = NewUser {
        email: EmailUtils::normalize_email(&payload.email),
        username: payload.username,
        password_hash: password_hash,
        bio: None,
        image_url: None,
    };

    let user = User::create(&pool, new_user).await?;

    // Generate tokens
    let jwt_manager = get_jwt_manager()?;
    let access_token = jwt_manager.generate_access_token(user.id)
        .map_err(|e| AppError::ValidationError(format!("Token generation failed: {}", e)))?;
    let refresh_token = jwt_manager.generate_refresh_token(user.id)
        .map_err(|e| AppError::ValidationError(format!("Token generation failed: {}", e)))?;

    let auth_response = AuthResponse {
        user: user.into(),
        access_token,
        refresh_token,
        expires_in: 15 * 60, // 15 minutes
    };

    Ok((StatusCode::CREATED, Json(auth_response)))
}

pub async fn login_handler(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    // Validate email format
    if !EmailUtils::is_valid_email(&payload.email) {
        return Err(AppError::ValidationError("Invalid email format".to_string()));
    }

    // Find user by email
    let user = User::find_by_email(&pool, &payload.email)
        .await?
        .ok_or(AppError::ValidationError("User not found".to_string()))?;

    // Verify password
    let is_valid = PasswordUtils::verify_password(&payload.password, &user.password_hash)
        .map_err(|e| AppError::ValidationError(format!("Password verification failed: {}", e)))?;

    if !is_valid {
        return Err(AppError::ValidationError("Invalid password".to_string()));
    }

    // Generate tokens
    let jwt_manager = get_jwt_manager()?;
    let access_token = jwt_manager.generate_access_token(user.id)
        .map_err(|e| AppError::ValidationError(format!("Token generation failed: {}", e)))?;
    let refresh_token = jwt_manager.generate_refresh_token(user.id)
        .map_err(|e| AppError::ValidationError(format!("Token generation failed: {}", e)))?;

    let auth_response = AuthResponse {
        user: user.into(),
        access_token,
        refresh_token,
        expires_in: 15 * 60, // 15 minutes
    };

    Ok(Json(auth_response))
}

pub async fn refresh_token_handler(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<RefreshTokenRequest>,
) -> Result<Json<TokenResponse>, AppError> {
    let jwt_manager = get_jwt_manager()?;

    // Verify refresh token
    let claims = jwt_manager.verify_refresh_token(&payload.refresh_token)
        .map_err(|_| AppError::Unauthorized)?;

    let user_id = claims.user_id()
        .map_err(|_| AppError::Unauthorized)?;

    // Verify user still exists
    let _user = User::find_by_id(&pool, user_id)
        .await?
        .ok_or(AppError::Unauthorized)?;

    // Generate new tokens
    let access_token = jwt_manager.generate_access_token(user_id)
        .map_err(|e| AppError::ValidationError(format!("Token generation failed: {}", e)))?;
    let refresh_token = jwt_manager.generate_refresh_token(user_id)
        .map_err(|e| AppError::ValidationError(format!("Token generation failed: {}", e)))?;

    let token_response = TokenResponse {
        access_token,
        refresh_token,
        expires_in: 15 * 60, // 15 minutes
    };

    Ok(Json(token_response))
}

pub async fn logout_handler() -> Result<Json<MessageResponse>, AppError> {
    // In a real application, you might want to blacklist the tokens
    // For now, we just return a success message
    Ok(Json(MessageResponse {
        message: "Successfully logged out".to_string(),
    }))
}

pub async fn me_handler(
    State(pool): State<Pool<Postgres>>,
    user: crate::middleware::AuthenticatedUser, // Use the AuthenticatedUser extractor
) -> Result<Json<UserResponse>, AppError> {
    let user = User::find_by_id(&pool, user.id)
        .await?
        .ok_or(AppError::Unauthorized)?;

    Ok(Json(user.into()))
}

fn validate_registration_input(payload: &RegisterRequest) -> Result<(), AppError> {
    let mut errors = Vec::new();

    // Validate email
    if !EmailUtils::is_valid_email(&payload.email) {
        errors.push("Invalid email format".to_string());
    }

    // Validate username
    if let Err(error) = UsernameUtils::validate_username_characters(&payload.username) {
        errors.push(error);
    }

    // Validate password strength
    if let Err(password_errors) = PasswordUtils::validate_password_strength(&payload.password) {
        errors.extend(password_errors);
    }

    if !errors.is_empty() {
        return Err(AppError::ValidationError(errors.join(", ")));
    }

    Ok(())
}

fn get_jwt_manager() -> Result<JwtManager, AppError> {
    let jwt_secret = std::env::var("JWT_SECRET")
        .map_err(|_| AppError::ValidationError("JWT_SECRET environment variable not set".to_string()))?;
    
    Ok(JwtManager::new(&jwt_secret))
}