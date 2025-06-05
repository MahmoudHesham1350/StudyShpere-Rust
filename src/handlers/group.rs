// src/handlers/group.rs
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
    models::group::{Group, NewGroup},
    middleware::auth::AuthenticatedUser,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGroupRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<Group> for GroupResponse {
    fn from(group: Group) -> Self {
        GroupResponse {
            id: group.id,
            name: group.name,
            description: group.description,
            created_at: group.created_at,
        }
    }
}

pub async fn create_group_handler(
    State(pool): State<Pool<Postgres>>,
    user: AuthenticatedUser,  // Add this parameter
    Json(payload): Json<CreateGroupRequest>,
) -> Result<(StatusCode, Json<GroupResponse>), AppError> {
    let new_group = NewGroup {
        name: payload.name,
        description: payload.description,
        owner_id: user.id,  // Use the authenticated user's ID
    };

    let group = Group::create(&pool, new_group).await?;

    Ok((StatusCode::CREATED, Json(group.into())))
}


pub async fn list_groups_handler(
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<Vec<GroupResponse>>, AppError> {
    let groups = Group::find_all(&pool).await?;
    let group_responses: Vec<GroupResponse> = groups.into_iter().map(Into::into).collect();
    Ok(Json(group_responses))
}