// src/handlers/join_request.rs
use axum::{
    extract::{State, Json, Path},
    http::StatusCode,
};
use sqlx::{Pool, Postgres};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::{
    errors::AppError,
    models,
    middleware::AuthenticatedUser,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponseForJoinRequest {
    pub id: Uuid,
    pub username: String,
}

impl From<models::user::User> for UserResponseForJoinRequest {
    fn from(user: models::user::User) -> Self {
        UserResponseForJoinRequest {
            id: user.id,
            username: user.username,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JoinRequestResponse {
    pub user: UserResponseForJoinRequest,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateJoinRequestPayload {
    pub user_id: Uuid, // This will likely come from authenticated user in real implementation
}

#[derive(Debug, Deserialize)]
pub struct RespondToJoinRequestPayload {
    pub group_name: String,
    pub user_id: Uuid,
    pub action: String, // "accept" or "decline"
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub message: String,
}

pub async fn list_join_requests_handler(
    State(pool): State<Pool<Postgres>>,
    Path(group_name): Path<String>,
) -> Result<Json<Vec<models::join_request::JoinRequestWithUser>>, AppError> {
    // // Check if group exists (optional, but good practice)
    // let _group = Group::find_by_name(&pool, group_name)
    //     .await?
    //     .ok_or(AppError::NotFound)?;

    let join_requests = models::join_request::JoinRequest::find_by_group_name(&pool, group_name).await?;
    Ok(Json(join_requests))
}

pub async fn create_join_request_handler(
    State(pool): State<Pool<Postgres>>,
    Path(group_name): Path<String>,
    user: AuthenticatedUser,
) -> Result<StatusCode, AppError> {
    // Check if group exists
    let _group = models::group::Group::find_by_name(&pool, group_name.clone())
        .await?
        .ok_or(AppError::NotFound)?;

    models::join_request::JoinRequest::create(&pool, models::join_request::JoinRequest {
        group_name: group_name,
        user_id: user.id,
        created_at: None,
    }).await?;

    Ok(StatusCode::CREATED)
}

pub async fn respond_to_join_request_handler(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<RespondToJoinRequestPayload>,
) -> Result<Json<MessageResponse>, AppError> {
    // // Verify group exists
    // let _group = Group::find_by_name(&pool, group_name.clone())
    //     .await?
    //     .ok_or(AppError::NotFound)?;

    models::join_request::JoinRequest::find_by_group_and_user(&pool, payload.group_name.clone(), payload.user_id)
        .await?
        .ok_or(AppError::NotFound)?;

    match payload.action.to_lowercase().as_str() {
        "accept" => {
            // Add user to group members (this would involve a new function in GroupMember model)
            // For now, just delete the join request
            models::join_request::JoinRequest::delete(&pool, payload.group_name.clone(), payload.user_id).await?;
            models::group_member::GroupMember::create(&pool, payload.user_id, payload.group_name).await?;
            Ok(Json(MessageResponse { message: "User added to group".to_string() }))
        }
        "decline" => {
            models::join_request::JoinRequest::delete(&pool, payload.group_name.clone(), payload.user_id).await?;
            Ok(Json(MessageResponse { message: "Join request declined".to_string() }))
        }
        _ => Err(AppError::ValidationError("Invalid action. Must be 'accept' or 'decline'".to_string())),
    }
}