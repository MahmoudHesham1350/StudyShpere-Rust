// src/routes/course.rs
use axum::{
    routing::get,
    Router,
};
use sqlx::PgPool;

use crate::handlers::course::{
    list_courses_handler,
    create_course_handler,
    get_course_detail_handler,
    update_course_handler,
    delete_course_handler,
};

pub fn course_routes() -> Router<PgPool> {
    Router::new()
        .route("/groups/{group_id}/courses", get(list_courses_handler).post(create_course_handler))
        .route("/courses/{course_id}", get(get_course_detail_handler).put(update_course_handler).delete(delete_course_handler))
}