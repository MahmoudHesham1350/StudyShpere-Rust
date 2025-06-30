// src/routes/course.rs
use axum::{
    routing::{get, post, put},
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
use crate::middleware::group_admin_middleware;

pub fn course_routes(pool: &PgPool) -> Router<PgPool> {
    Router::new()
        .route("/groups/{group_name}/courses", get(list_courses_handler))
        .route("/groups/{group_name}/courses", 
            post(create_course_handler)
        .layer(axum::middleware::from_fn_with_state(pool.clone(),group_admin_middleware))
        )

        .route("/groups/{group_name}/courses/{course_name}", get(get_course_detail_handler))

        .route("/groups/{group_name}/courses/{course_name}", 
put(update_course_handler)
        .delete(delete_course_handler)
            .layer(axum::middleware::from_fn_with_state(pool.clone(), group_admin_middleware))
        )
}