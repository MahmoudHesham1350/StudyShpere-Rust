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
    models::{course::{Course, NewCourse}, group::Group},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CourseResponse {
    pub name: String,
    pub group_name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<Course> for CourseResponse {
    fn from(course: Course) -> Self {
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
    Path(group_name): Path<&str>,
) -> Result<Json<Vec<CourseResponse>>, AppError> {
    let _group = Group::find_by_name(&pool, group_name)
        .await?
        .ok_or(AppError::NotFound)?;

    let courses = Course::find_by_group_name(&pool, group_name).await?;
    let responses: Vec<CourseResponse> = courses.into_iter().map(Into::into).collect();
    Ok(Json(responses))
}

pub async fn create_course_handler(
    State(pool): State<Pool<Postgres>>,
    Path(group_name): Path<&str>,
    Json(payload): Json<CreateCoursePayload>,
) -> Result<(StatusCode, Json<CourseResponse>), AppError> {
    let _group = Group::find_by_name(&pool, group_name)
        .await?
        .ok_or(AppError::NotFound)?;

    let new_course = NewCourse {
        group_name: group_name.to_string(),
        name: payload.name,
        description: payload.description,
    };

    let course = Course::create(&pool, new_course).await?;
    Ok((StatusCode::CREATED, Json(course.into())))
}

pub async fn get_course_detail_handler(
    State(pool): State<Pool<Postgres>>,
    Path(group_name): Path<&str>,
    Path(course_name): Path<&str>,
) -> Result<Json<CourseResponse>, AppError> {
    let course = Course::find_by_group_and_name(&pool, group_name, course_name)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(course.into()))
}

pub async fn update_course_handler(
    State(pool): State<Pool<Postgres>>,
    Path(group_name): Path<&str>,
    Path(course_name): Path<&str>,
    Json(payload): Json<UpdateCoursePayload>,
) -> Result<Json<CourseResponse>, AppError> {
    let mut course = Course::find_by_group_and_name(&pool, group_name, course_name)
        .await?
        .ok_or(AppError::NotFound)?;

    if let Some(name) = payload.name {
        course.name = name;
    }
    if let Some(description) = payload.description {
        course.description = Some(description);
    }

    let updated_course = Course::update(&pool, group_name, course_name, course).await?;
    Ok(Json(updated_course.into()))
}

pub async fn delete_course_handler(
    State(pool): State<Pool<Postgres>>,
    Path(group_name): Path<&str>,
    Path(course_name): Path<&str>,
) -> Result<StatusCode, AppError> {
    let _course = Course::find_by_group_and_name(&pool, group_name, course_name)
        .await?
        .ok_or(AppError::NotFound)?;

    Course::delete(&pool, group_name, course_name).await?;
    Ok(StatusCode::NO_CONTENT)
}