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
    errors::AppError, middleware::AuthenticatedUser,
    models
};


#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponseForGroupMember {
    pub id: Uuid,
    pub username: String,
}

impl From<models::user::User> for UserResponseForGroupMember {
    fn from(user: models::user::User) -> Self {
        UserResponseForGroupMember {
            id: user.id,
            username: user.username,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupMemberResponse {
    pub user: UserResponseForGroupMember,
    pub user_role: Option<String>,
    pub joined_at: DateTime<Utc>,
}


#[derive(Debug, Deserialize)]
pub struct CreateGroupMemberPayload {
    pub user_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct UpdateGroupMemberPayload {
    pub user_role: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteGroupMemberPayload {
    pub user_id: Uuid,
}

pub async fn list_group_members_handler(
    State(pool): State<Pool<Postgres>>,
    Path(group_name): Path<String>,
) -> Result<Json<Vec<models::group_member::GroupMemberWithUser>>, AppError> {
    let group_members = models::group_member::GroupMember::find_by_group_name(&pool, group_name).await?;
    Ok(Json(group_members))
}

pub async fn create_group_member_handler(
    State(pool): State<Pool<Postgres>>,
    Path(group_name): Path<String>,
    Json(payload): Json<CreateGroupMemberPayload>,
) -> Result<StatusCode, AppError> {
    let _group = models::group::Group::find_by_name(&pool, group_name.clone())
        .await?
        .ok_or(AppError::NotFound)?;

    models::user::User::find_by_id(&pool, payload.user_id)
        .await?
        .ok_or(AppError::NotFound)?;

    models::group_member::GroupMember::create(&pool, payload.user_id, group_name).await?;

    Ok(StatusCode::CREATED)
}



// TODO: Implement update_group_member_handler after implementing member types
// pub async fn update_group_member_handler(
//     State(pool): State<Pool<Postgres>>,
//     Path((group_name, user_id)): Path<(String, Uuid)>,
//     Json(payload): Json<UpdateGroupMemberPayload>,
// ) -> Result<Json<GroupMemberResponse>, AppError> {
//     let _group = models::group::Group::find_by_name(&pool, group_name)
//         .await?
//         .ok_or(AppError::NotFound)?;

//     let mut group_member = models::group_member::GroupMember::find_by_user_and_group(&pool, user_id, group_name)
//         .await?
//         .ok_or(AppError::NotFound)?;

//     group_member.user_role = payload.user_role;

//     let updated_group_member = models::group_member::GroupMember::update(&pool, group_member).await?;

//     Ok(Json(GroupMemberResponse::from_group_member(&pool, updated_group_member).await?))
// }

pub async fn delete_group_member_handler(
    State(pool): State<Pool<Postgres>>,
    Path(group_name): Path<String>,
    Json(payload): Json<DeleteGroupMemberPayload>,
) -> Result<StatusCode, AppError> {
    let _group_member = models::group_member::GroupMember::find_by_user_and_group(&pool, payload.user_id, group_name.clone())
        .await?
        .ok_or(AppError::NotFound)?;

    models::group_member::GroupMember::delete(&pool, payload.user_id, group_name).await?;

    Ok(StatusCode::NO_CONTENT)
}

// // Handler for /api/groups/<uuid:group_name>/members/self/
// pub async fn get_self_group_membership_handler(
//     State(pool): State<Pool<Postgres>>,
//     user: AuthenticatedUser,
//     Path(group_name): Path<String>,
//     // TODO: Extract authenticated user ID here
// ) -> Result<Json<GroupMemberResponse>, AppError> {
//     let _group = models::group::Group::find_by_name(&pool, group_name)
//         .await?
//         .ok_or(AppError::NotFound)?;

//     // Placeholder for authenticated user ID
//     let current_user_id = user.id;

//     let group_member = models::group_member::GroupMember::find_by_user_and_group_name(&pool, current_user_id, group_name)
//         .await?
//         .ok_or(AppError::NotFound)?;

//     Ok(Json(GroupMemberResponse::from_group_member(&pool, group_member).await?))
// }

pub async fn leave_group_membership_handler(
    State(pool): State<Pool<Postgres>>,
    user: AuthenticatedUser,
    Path(group_name): Path<String>,
) -> Result<StatusCode, AppError> {

    let _group_member = models::group_member::GroupMember::find_by_user_and_group(&pool, user.id, group_name.clone())
        .await?
        .ok_or(AppError::NotFound)?;

    models::group_member::GroupMember::delete(&pool, user.id, group_name).await?;

    Ok(StatusCode::NO_CONTENT)
}