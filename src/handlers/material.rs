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
    models::material::{Material, NewMaterial},
    models::material_label::MaterialLabel,
    models::comment::{Comment, NewComment},
};

// Material-related request/response DTOs
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMaterialRequest {
    pub title: String,
    pub file: Option<String>,
    pub url: Option<String>,
    pub material_type: String,
    pub course_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMaterialRequest {
    pub title: String,
    pub file: Option<String>,
    pub url: Option<String>,
    pub material_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialResponse {
    pub id: Uuid,
    pub title: String,
    pub file: Option<String>,
    pub url: Option<String>,
    pub material_type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub owner_id: Uuid,
    pub course_id: Uuid,
}

impl From<Material> for MaterialResponse {
    fn from(material: Material) -> Self {
        MaterialResponse {
            id: material.id,
            title: material.title,
            file: material.file,
            url: material.url,
            material_type: material.material_type,
            created_at: material.created_at,
            updated_at: material.updated_at,
            owner_id: material.owner_id,
            course_id: material.course_id,
        }
    }
}

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
    pub id: Uuid,
    pub material_id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

impl From<Comment> for CommentResponse {
    fn from(comment: Comment) -> Self {
        CommentResponse {
            id: comment.id,
            material_id: comment.material_id,
            user_id: comment.user_id,
            content: comment.content,
            created_at: comment.created_at,
        }
    }
}

// MaterialLabel-related request/response DTOs
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMaterialLabelRequest {
    pub label_id: Uuid,
    pub number: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialLabelResponse {
    pub material_id: Uuid,
    pub label_id: Uuid,
    pub number: i32,
}

impl From<MaterialLabel> for MaterialLabelResponse {
    fn from(label: MaterialLabel) -> Self {
        MaterialLabelResponse {
            material_id: label.material_id,
            label_id: label.label_id,
            number: label.number,
        }
    }
}

// Material handlers
pub async fn create_material_handler(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<CreateMaterialRequest>,
) -> Result<(StatusCode, Json<MaterialResponse>), AppError> {
    let new_material = NewMaterial {
        title: payload.title,
        file: payload.file,
        url: payload.url,
        material_type: payload.material_type,
        owner_id: Uuid::new_v4(), // Placeholder for now, will be replaced with actual user ID
        course_id: payload.course_id,
    };

    let material = Material::create(&pool, new_material).await?;
    Ok((StatusCode::CREATED, Json(material.into())))
}

pub async fn list_materials_by_course_handler(
    State(pool): State<Pool<Postgres>>,
    Path(course_id): Path<Uuid>,
) -> Result<Json<Vec<MaterialResponse>>, AppError> {
    let materials = Material::find_by_course_id(&pool, course_id).await?;
    let responses: Vec<MaterialResponse> = materials.into_iter().map(Into::into).collect();
    Ok(Json(responses))
}

pub async fn get_material_handler(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<Uuid>,
) -> Result<Json<MaterialResponse>, AppError> {
    let material = Material::find_by_id(&pool, id)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(material.into()))
}

pub async fn update_material_handler(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateMaterialRequest>,
) -> Result<Json<MaterialResponse>, AppError> {
    let material = Material::update(
        &pool,
        id,
        payload.title,
        payload.file,
        payload.url,
        payload.material_type,
    ).await?;
    Ok(Json(material.into()))
}

pub async fn delete_material_handler(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    Material::delete(&pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// Comment handlers
pub async fn create_comment_handler(
    State(pool): State<Pool<Postgres>>,
    Path(material_id): Path<Uuid>,
    Json(payload): Json<CreateCommentRequest>,
) -> Result<(StatusCode, Json<CommentResponse>), AppError> {
    let new_comment = NewComment {
        material_id,
        user_id: Uuid::new_v4(), // Placeholder for now, will be replaced with actual user ID
        content: payload.content,
    };

    let comment = Comment::create(&pool, new_comment).await?;
    Ok((StatusCode::CREATED, Json(comment.into())))
}

pub async fn list_comments_handler(
    State(pool): State<Pool<Postgres>>,
    Path(material_id): Path<Uuid>,
) -> Result<Json<Vec<CommentResponse>>, AppError> {
    let comments = Comment::find_by_material_id(&pool, material_id).await?;
    let responses: Vec<CommentResponse> = comments.into_iter().map(Into::into).collect();
    Ok(Json(responses))
}

pub async fn get_comment_handler(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<Uuid>,
) -> Result<Json<CommentResponse>, AppError> {
    let comment = Comment::find_by_id(&pool, id)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(comment.into()))
}

pub async fn update_comment_handler(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateCommentRequest>,
) -> Result<Json<CommentResponse>, AppError> {
    let comment = Comment::update(&pool, id, payload.content).await?;
    Ok(Json(comment.into()))
}

pub async fn delete_comment_handler(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    Comment::delete(&pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// MaterialLabel handlers
pub async fn create_material_label_handler(
    State(pool): State<Pool<Postgres>>,
    Path(material_id): Path<Uuid>,
    Json(payload): Json<CreateMaterialLabelRequest>,
) -> Result<(StatusCode, Json<MaterialLabelResponse>), AppError> {
    let material_label = MaterialLabel::create(
        &pool,
        material_id,
        payload.label_id,
        payload.number,
    ).await?;
    Ok((StatusCode::CREATED, Json(material_label.into())))
}

pub async fn list_material_labels_handler(
    State(pool): State<Pool<Postgres>>,
    Path(material_id): Path<Uuid>,
) -> Result<Json<Vec<MaterialLabelResponse>>, AppError> {
    let labels = MaterialLabel::find_by_material_id(&pool, material_id).await?;
    let responses: Vec<MaterialLabelResponse> = labels.into_iter().map(Into::into).collect();
    Ok(Json(responses))
}

pub async fn delete_material_label_handler(
    State(pool): State<Pool<Postgres>>,
    Path((material_id, label_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, AppError> {
    MaterialLabel::delete(&pool, material_id, label_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
