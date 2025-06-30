// src/handlers/course.rs
use axum::{
    extract::{State, Json, Path},
    http::StatusCode,
};
use sqlx::{Pool, Postgres};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

use crate::{
    errors::AppError,
    models,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CourseResponse {
    pub name: String,
    pub group_name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<models::course::Course> for CourseResponse {
    fn from(course: models::course::Course) -> Self {
        CourseResponse {
            name: course.name,
            group_name: course.group_name,
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
    Path(group_name): Path<String>,
) -> Result<Json<Vec<CourseResponse>>, AppError> {
    let _group = models::group::Group::find_by_name(&pool, group_name.clone())
        .await?
        .ok_or(AppError::NotFound)?;

    let courses = models::course::Course::find_by_group_name(&pool, group_name).await?;
    let responses: Vec<CourseResponse> = courses.into_iter().map(Into::into).collect();
    Ok(Json(responses))
}

pub async fn create_course_handler(
    State(pool): State<Pool<Postgres>>,
    Path(group_name): Path<String>,
    Json(payload): Json<CreateCoursePayload>,
) -> Result<(StatusCode, Json<CourseResponse>), AppError> {
    let _group = models::group::Group::find_by_name(&pool, group_name.clone())
        .await?
        .ok_or(AppError::NotFound)?;

    let new_course = models::course::NewCourse {
        group_name: group_name.to_string(),
        name: payload.name,
        description: payload.description,
    };

    let course = models::course::Course::create(&pool, new_course).await?;
    Ok((StatusCode::CREATED, Json(course.into())))
}

pub async fn get_course_detail_handler(
    State(pool): State<Pool<Postgres>>,
    Path(group_name): Path<String>,
    Path(course_name): Path<String>,
) -> Result<Json<CourseResponse>, AppError> {
    let course = models::course::Course::find_by_group_and_name(&pool, group_name, course_name)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(course.into()))
}

pub async fn update_course_handler(
    State(pool): State<Pool<Postgres>>,
    Path(group_name): Path<String>,
    Path(course_name): Path<String>,
    Json(payload): Json<UpdateCoursePayload>,
) -> Result<Json<CourseResponse>, AppError> {
    let mut course = models::course::Course::find_by_group_and_name(&pool, group_name.clone(), course_name.clone())
        .await?
        .ok_or(AppError::NotFound)?;

    if let Some(name) = payload.name {
        course.name = name;
    }
    if let Some(description) = payload.description {
        course.description = Some(description);
    }

    let updated_course = models::course::Course::update(&pool, group_name, course_name, course).await?;
    Ok(Json(updated_course.into()))
}

pub async fn delete_course_handler(
    State(pool): State<Pool<Postgres>>,
    Path(group_name): Path<String>,
    Path(course_name): Path<String>,
) -> Result<StatusCode, AppError> {
    let _course = models::course::Course::find_by_group_and_name(&pool, group_name.clone(), course_name.clone())
        .await?
        .ok_or(AppError::NotFound)?;

    models::course::Course::delete(&pool, group_name, course_name).await?;
    Ok(StatusCode::NO_CONTENT)
}