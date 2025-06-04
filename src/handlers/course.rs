// src/handlers/course.rs
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
    models::course::{Course, NewCourse},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CourseResponse {
    pub id: Uuid,
    pub group_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<Course> for CourseResponse {
    fn from(course: Course) -> Self {
        CourseResponse {
            id: course.id,
            group_id: course.group_id,
            name: course.name,
            description: course.description,
            created_at: course.created_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateCoursePayload {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCoursePayload {
    pub name: Option<String>,
    pub description: Option<String>,
}

pub async fn list_courses_handler(
    State(pool): State<Pool<Postgres>>,
    Path(group_id): Path<Uuid>,
) -> Result<Json<Vec<CourseResponse>>, AppError> {
    let _group = Group::find_by_id(&pool, group_id)
        .await?
        .ok_or(AppError::NotFound)?;

    let courses = Course::find_by_group_id(&pool, group_id).await?;
    let responses: Vec<CourseResponse> = courses.into_iter().map(Into::into).collect();
    Ok(Json(responses))
}

pub async fn create_course_handler(
    State(pool): State<Pool<Postgres>>,
    Path(group_id): Path<Uuid>,
    Json(payload): Json<CreateCoursePayload>,
) -> Result<(StatusCode, Json<CourseResponse>), AppError> {
    let _group = Group::find_by_id(&pool, group_id)
        .await?
        .ok_or(AppError::NotFound)?;

    let new_course = NewCourse {
        group_id,
        name: payload.name,
        description: payload.description,
    };

    let course = Course::create(&pool, new_course).await?;
    Ok((StatusCode::CREATED, Json(course.into())))
}

pub async fn get_course_detail_handler(
    State(pool): State<Pool<Postgres>>,
    Path(course_id): Path<Uuid>,
) -> Result<Json<CourseResponse>, AppError> {
    let course = Course::find_by_id(&pool, course_id)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(course.into()))
}

pub async fn update_course_handler(
    State(pool): State<Pool<Postgres>>,
    Path(course_id): Path<Uuid>,
    Json(payload): Json<UpdateCoursePayload>,
) -> Result<Json<CourseResponse>, AppError> {
    let mut course = Course::find_by_id(&pool, course_id)
        .await?
        .ok_or(AppError::NotFound)?;

    if let Some(name) = payload.name {
        course.name = name;
    }
    if let Some(description) = payload.description {
        course.description = Some(description);
    }

    let updated_course = Course::update(&pool, course).await?;
    Ok(Json(updated_course.into()))
}

pub async fn delete_course_handler(
    State(pool): State<Pool<Postgres>>,
    Path(course_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let _course = Course::find_by_id(&pool, course_id)
        .await?
        .ok_or(AppError::NotFound)?;

    Course::delete(&pool, course_id).await?;
    Ok(StatusCode::NO_CONTENT)
}