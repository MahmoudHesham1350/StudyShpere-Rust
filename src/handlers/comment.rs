use axum::{
    extract::{State, Json, Path},
    http::StatusCode,
};
use sqlx::{Pool, Postgres};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{Utc, DateTime};

use crate::{
    errors::AppError, middleware::auth::AuthenticatedUser, models
};




// Comment-related request/response DTOs
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCommentRequest {
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCommentRequest {
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentResponse {
    pub id: i32,
    pub material_id: Uuid,
    pub user_id: Option<Uuid>,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

impl From<models::comment::Comment> for CommentResponse {
    fn from(comment: models::comment::Comment) -> Self {
        CommentResponse {
            id: comment.id,
            material_id: comment.material_id,
            user_id: comment.user_id,
            content: comment.content,
            created_at: comment.created_at,
        }
    }
}

// Comment handlers
pub async fn create_comment_handler(
    State(pool): State<Pool<Postgres>>,
    user: AuthenticatedUser,
    Path(material_id): Path<Uuid>,
    Json(payload): Json<CreateCommentRequest>,
) -> Result<(StatusCode, Json<CommentResponse>), AppError> {
    let new_comment = models::comment::NewComment {
        material_id,
        user_id: user.id, // Placeholder for now, will be replaced with actual user ID
        content: payload.content,
    };

    let comment = models::comment::Comment::create(&pool, new_comment).await?;
    Ok((StatusCode::CREATED, Json(comment.into())))
}



pub async fn list_comments_handler(
    State(pool): State<Pool<Postgres>>,
    Path(material_id): Path<Uuid>,
) -> Result<Json<Vec<CommentResponse>>, AppError> {
    let comments = models::comment::Comment::find_by_material_id(&pool, material_id).await?;
    let responses: Vec<CommentResponse> = comments.into_iter().map(Into::into).collect();
    Ok(Json(responses))
}

pub async fn get_comment_handler(
    State(pool): State<Pool<Postgres>>,
    Path(material_id): Path<Uuid>,
    Path(id): Path<i32>,
) -> Result<Json<CommentResponse>, AppError> {
    let comment = models::comment::Comment::find_by_id(&pool, material_id, id)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(comment.into()))
}

pub async fn update_comment_handler(
    State(pool): State<Pool<Postgres>>,
    Path(material_id): Path<Uuid>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateCommentRequest>,
) -> Result<Json<CommentResponse>, AppError> {
    let comment = models::comment::Comment::update(&pool, material_id, id, payload.content).await?;
    Ok(Json(comment.into()))
}

pub async fn delete_comment_handler(
    State(pool): State<Pool<Postgres>>,
    Path(material_id): Path<Uuid>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    models::comment::Comment::delete(&pool, material_id, id).await?;
    Ok(StatusCode::NO_CONTENT)
}





