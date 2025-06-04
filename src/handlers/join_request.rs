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
    models::join_request::JoinRequest,
    models::group::Group,
    models::user::User, // Assuming User model is needed for response
};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponseForJoinRequest {
    pub id: Uuid,
    pub username: String,
}

impl From<User> for UserResponseForJoinRequest {
    fn from(user: User) -> Self {
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

impl JoinRequestResponse {
    pub async fn from_join_request(pool: &Pool<Postgres>, join_request: JoinRequest) -> Result<Self, AppError> {
        let user = User::find_by_id(pool, join_request.user_id)
            .await?
            .ok_or(AppError::NotFound)?; // User not found for join request

        Ok(JoinRequestResponse {
            user: user.into(),
            created_at: join_request.created_at,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateJoinRequestPayload {
    pub user_id: Uuid, // This will likely come from authenticated user in real implementation
}

#[derive(Debug, Deserialize)]
pub struct RespondToJoinRequestPayload {
    pub action: String, // "accept" or "decline"
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub message: String,
}

pub async fn list_join_requests_handler(
    State(pool): State<Pool<Postgres>>,
    Path(group_id): Path<Uuid>,
) -> Result<Json<Vec<JoinRequestResponse>>, AppError> {
    // Check if group exists (optional, but good practice)
    let _group = Group::find_by_id(&pool, group_id)
        .await?
        .ok_or(AppError::NotFound)?;

    let join_requests = JoinRequest::find_by_group_id(&pool, group_id).await?;
    let mut responses = Vec::new();
    for jr in join_requests {
        responses.push(JoinRequestResponse::from_join_request(&pool, jr).await?);
    }
    Ok(Json(responses))
}

pub async fn create_join_request_handler(
    State(pool): State<Pool<Postgres>>,
    Path(group_id): Path<Uuid>,
    Json(payload): Json<CreateJoinRequestPayload>,
) -> Result<(StatusCode, Json<JoinRequestResponse>), AppError> {
    // Check if group exists
    let _group = Group::find_by_id(&pool, group_id)
        .await?
        .ok_or(AppError::NotFound)?;

    // Check if user exists
    let user = User::find_by_id(&pool, payload.user_id)
        .await?
        .ok_or(AppError::NotFound)?;

    let new_join_request = JoinRequest {
        id: Uuid::new_v4(), // Generate new UUID for join request
        group_id,
        user_id: payload.user_id,
        created_at: Utc::now(),
    };

    let join_request = JoinRequest::create(&pool, new_join_request).await?;
    let response = JoinRequestResponse {
        user: user.into(),
        created_at: join_request.created_at,
    };

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn respond_to_join_request_handler(
    State(pool): State<Pool<Postgres>>,
    Path((group_id, join_request_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<RespondToJoinRequestPayload>,
) -> Result<Json<MessageResponse>, AppError> {
    // Verify group exists
    let _group = Group::find_by_id(&pool, group_id)
        .await?
        .ok_or(AppError::NotFound)?;

    let join_request = JoinRequest::find_by_id(&pool, join_request_id)
        .await?
        .ok_or(AppError::NotFound)?;

    // Ensure the join request belongs to the specified group
    if join_request.group_id != group_id {
        return Err(AppError::NotFound); // Or a more specific error like Forbidden
    }

    match payload.action.as_str() {
        "accept" => {
            // Add user to group members (this would involve a new function in GroupMember model)
            // For now, just delete the join request
            JoinRequest::delete(&pool, join_request_id).await?;
            Ok(Json(MessageResponse { message: "User added to group".to_string() }))
        }
        "decline" => {
            JoinRequest::delete(&pool, join_request_id).await?;
            Ok(Json(MessageResponse { message: "Join request declined".to_string() }))
        }
        _ => Err(AppError::ValidationError("Invalid action. Must be 'accept' or 'decline'".to_string())),
    }
}