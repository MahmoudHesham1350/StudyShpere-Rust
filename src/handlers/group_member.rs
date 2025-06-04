// src/handlers/group_member.rs
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
    models::group::Group,
    models::group_member::GroupMember,
    models::user::User,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponseForGroupMember {
    pub id: Uuid,
    pub username: String,
}

impl From<User> for UserResponseForGroupMember {
    fn from(user: User) -> Self {
        UserResponseForGroupMember {
            id: user.id,
            username: user.username,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupMemberResponse {
    pub user: UserResponseForGroupMember,
    pub user_role: String,
    pub joined_at: DateTime<Utc>,
}

impl GroupMemberResponse {
    pub async fn from_group_member(pool: &Pool<Postgres>, group_member: GroupMember) -> Result<Self, AppError> {
        let user = User::find_by_id(pool, group_member.user_id)
            .await?
            .ok_or(AppError::NotFound)?;

        Ok(GroupMemberResponse {
            user: user.into(),
            user_role: group_member.user_role,
            joined_at: group_member.joined_at,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateGroupMemberPayload {
    pub user_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct UpdateGroupMemberPayload {
    pub user_role: String,
}

pub async fn list_group_members_handler(
    State(pool): State<Pool<Postgres>>,
    Path(group_id): Path<Uuid>,
) -> Result<Json<Vec<GroupMemberResponse>>, AppError> {
    let _group = Group::find_by_id(&pool, group_id)
        .await?
        .ok_or(AppError::NotFound)?;

    let group_members = GroupMember::find_by_group_id(&pool, group_id).await?;
    let mut responses = Vec::new();
    for gm in group_members {
        responses.push(GroupMemberResponse::from_group_member(&pool, gm).await?);
    }
    Ok(Json(responses))
}

pub async fn create_group_member_handler(
    State(pool): State<Pool<Postgres>>,
    Path(group_id): Path<Uuid>,
    Json(payload): Json<CreateGroupMemberPayload>,
) -> Result<(StatusCode, Json<GroupMemberResponse>), AppError> {
    let _group = Group::find_by_id(&pool, group_id)
        .await?
        .ok_or(AppError::NotFound)?;

    let user = User::find_by_id(&pool, payload.user_id)
        .await?
        .ok_or(AppError::NotFound)?;

    let new_group_member = GroupMember {
        user_id: payload.user_id,
        group_id,
        user_role: "member".to_string(), // Default role
        joined_at: Utc::now(),
    };

    let group_member = GroupMember::create(&pool, new_group_member).await?;
    let response = GroupMemberResponse::from_group_member(&pool, group_member).await?;

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn get_group_member_detail_handler(
    State(pool): State<Pool<Postgres>>,
    Path((group_id, user_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<GroupMemberResponse>, AppError> {
    let _group = Group::find_by_id(&pool, group_id)
        .await?
        .ok_or(AppError::NotFound)?;

    let group_member = GroupMember::find_by_user_and_group_id(&pool, user_id, group_id)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Json(GroupMemberResponse::from_group_member(&pool, group_member).await?))
}

pub async fn update_group_member_handler(
    State(pool): State<Pool<Postgres>>,
    Path((group_id, user_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<UpdateGroupMemberPayload>,
) -> Result<Json<GroupMemberResponse>, AppError> {
    let _group = Group::find_by_id(&pool, group_id)
        .await?
        .ok_or(AppError::NotFound)?;

    let mut group_member = GroupMember::find_by_user_and_group_id(&pool, user_id, group_id)
        .await?
        .ok_or(AppError::NotFound)?;

    group_member.user_role = payload.user_role;

    let updated_group_member = GroupMember::update(&pool, group_member).await?;

    Ok(Json(GroupMemberResponse::from_group_member(&pool, updated_group_member).await?))
}

pub async fn delete_group_member_handler(
    State(pool): State<Pool<Postgres>>,
    Path((group_id, user_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, AppError> {
    let _group = Group::find_by_id(&pool, group_id)
        .await?
        .ok_or(AppError::NotFound)?;

    let _group_member = GroupMember::find_by_user_and_group_id(&pool, user_id, group_id)
        .await?
        .ok_or(AppError::NotFound)?;

    GroupMember::delete(&pool, user_id, group_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

// Handler for /api/groups/<uuid:group_id>/members/self/
pub async fn get_self_group_membership_handler(
    State(pool): State<Pool<Postgres>>,
    Path(group_id): Path<Uuid>,
    // TODO: Extract authenticated user ID here
) -> Result<Json<GroupMemberResponse>, AppError> {
    let _group = Group::find_by_id(&pool, group_id)
        .await?
        .ok_or(AppError::NotFound)?;

    // Placeholder for authenticated user ID
    let current_user_id = Uuid::new_v4(); // Replace with actual authenticated user ID

    let group_member = GroupMember::find_by_user_and_group_id(&pool, current_user_id, group_id)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Json(GroupMemberResponse::from_group_member(&pool, group_member).await?))
}

pub async fn leave_group_membership_handler(
    State(pool): State<Pool<Postgres>>,
    Path(group_id): Path<Uuid>,
    // TODO: Extract authenticated user ID here
) -> Result<StatusCode, AppError> {
    let _group = Group::find_by_id(&pool, group_id)
        .await?
        .ok_or(AppError::NotFound)?;

    // Placeholder for authenticated user ID
    let current_user_id = Uuid::new_v4(); // Replace with actual authenticated user ID

    let _group_member = GroupMember::find_by_user_and_group_id(&pool, current_user_id, group_id)
        .await?
        .ok_or(AppError::NotFound)?;

    GroupMember::delete(&pool, current_user_id, group_id).await?;

    Ok(StatusCode::NO_CONTENT)
}