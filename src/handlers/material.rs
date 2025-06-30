use axum::{
    extract::{State, Json, Path},
    http::StatusCode,
};
use sqlx::{Pool, Postgres};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::{
    errors::AppError, middleware::AuthenticatedUser, models
};

// Material-related request/response DTOs
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMaterialRequest {
    pub group_name: String,
    pub course_name: String,
    pub title: String,
    pub file: Option<String>,
    pub url: Option<String>,
    pub material_type: String,
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
    pub group_name: String,
    pub course_name: String,
    
    pub title: String,
    pub file: Option<String>,
    pub url: Option<String>,
    pub material_type: String,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    
    pub creator_id: Uuid,
}

impl From<models::material::Material> for MaterialResponse {
    fn from(material: models::material::Material) -> Self {
        MaterialResponse {
            id: material.id,
            group_name: material.group_name,
            course_name: material.course_name,
            
            title: material.title,
            file: material.file,
            url: material.url,
            material_type: material.material_type,
            
            created_at: material.created_at,
            updated_at: material.updated_at,
            
            creator_id: material.creator,
        }
    }
}

// Material handlers
pub async fn create_material_handler(
    State(pool): State<Pool<Postgres>>,
    user: AuthenticatedUser,  
    Json(payload): Json<CreateMaterialRequest>,
) -> Result<(StatusCode, Json<MaterialResponse>), AppError> {
    dbg!(user.id);
    let new_material = models::material::NewMaterial {
        group_name : payload.group_name,
        course_name : payload.course_name,
        title : payload.title,
        file : payload.file,
        url : payload.url,
        material_type: payload.material_type,
        creator : user.id
    };

    let material = models::material::Material::create(&pool, new_material).await?;
    Ok((StatusCode::CREATED, Json(material.into())))
}

pub async fn list_materials_by_course_handler(
    State(pool): State<Pool<Postgres>>,
    Path(group_name): Path<String>,
    Path(course_name): Path<String>,
) -> Result<Json<Vec<MaterialResponse>>, AppError> {
    let materials = models::material::Material::find_by_course(&pool, group_name, course_name).await?;
    let responses: Vec<MaterialResponse> = materials.into_iter().map(Into::into).collect();
    Ok(Json(responses))
}

pub async fn get_material_handler(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<Uuid>,
) -> Result<Json<MaterialResponse>, AppError> {
    let material = models::material::Material::find_by_id(&pool, id)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(material.into()))
}

pub async fn update_material_handler(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateMaterialRequest>,
) -> Result<Json<MaterialResponse>, AppError> {
    let material = models::material::Material::update(
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
    models::material::Material::delete(&pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

