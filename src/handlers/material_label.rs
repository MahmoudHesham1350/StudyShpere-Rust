use axum::{
    extract::{State, Json, Path},
    http::StatusCode,
};
use sqlx::{Pool, Postgres};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{
    errors::AppError, models
};


// MaterialLabel-related request/response DTOs
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMaterialLabelRequest {
    pub label_name: String,
    pub group_name: String,
    pub number: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialLabelResponse {
    pub material_id: Uuid,
    pub label_name: String,
    pub number: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMaterialLabelRequest {
    pub label_name: String,
    pub group_name: String,
    pub number: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteMaterialLabelRequest {
    pub label_name: String,
    pub group_name: String,
}

impl From<models::material_label::MaterialLabel> for MaterialLabelResponse {
    fn from(label: models::material_label::MaterialLabel) -> Self {
        MaterialLabelResponse {
            material_id: label.material_id,
            label_name: label.label_name,
            number: label.number,
        }
    }
}


// MaterialLabel handlers
pub async fn create_material_label_handler(
    State(pool): State<Pool<Postgres>>,
    Path(material_id): Path<Uuid>,
    Json(payload): Json<CreateMaterialLabelRequest>,
) -> Result<(StatusCode, Json<MaterialLabelResponse>), AppError> {
    let material_label = models::material_label::MaterialLabel::create(
        &pool,
        models::material_label::MaterialLabel { 
            material_id: material_id, 
            group_name: payload.group_name, 
            label_name: payload.label_name, 
            number: payload.number 
        }
    ).await?;
    Ok((StatusCode::CREATED, Json(material_label.into())))
}

pub async fn list_material_labels_handler(
    State(pool): State<Pool<Postgres>>,
    Path(material_id): Path<Uuid>,
) -> Result<Json<Vec<MaterialLabelResponse>>, AppError> {
    let labels = models::material_label::MaterialLabel::find_by_material_id(&pool, material_id).await?;
    let responses: Vec<MaterialLabelResponse> = labels.into_iter().map(Into::into).collect();
    Ok(Json(responses))
}

pub async fn delete_material_label_handler(
    State(pool): State<Pool<Postgres>>,
    Path(material_id): Path<Uuid>,
    Json(label_name): Json<String>
) -> Result<StatusCode, AppError> {
    models::material_label::MaterialLabel::delete(&pool, material_id, label_name).await?;
    Ok(StatusCode::NO_CONTENT)
}
    