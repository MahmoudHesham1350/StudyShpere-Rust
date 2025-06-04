use axum::{
    routing::{get, post, put, delete},
    Router,
};
use sqlx::PgPool;

use crate::handlers::material::{
    // Material handlers
    create_material_handler,
    list_materials_by_course_handler,
    get_material_handler,
    update_material_handler,
    delete_material_handler,
    // Comment handlers
    create_comment_handler,
    list_comments_handler,
    get_comment_handler,
    update_comment_handler,
    delete_comment_handler,
    // MaterialLabel handlers
    create_material_label_handler,
    list_material_labels_handler,
    delete_material_label_handler,
};

pub fn material_routes() -> Router<PgPool> {
    Router::new()
        // Material routes
        .route("/materials", post(create_material_handler))
        .route("/courses/{course_id}/materials", get(list_materials_by_course_handler))
        .route("/materials/{id}", get(get_material_handler))
        .route("/materials/{id}", put(update_material_handler))
        .route("/materials/{id}", delete(delete_material_handler))
        
        // Comment routes
        .route("/materials/{material_id}/comments", post(create_comment_handler))
        .route("/materials/{material_id}/comments", get(list_comments_handler))
        .route("/comments/{id}", get(get_comment_handler))
        .route("/comments/{id}", put(update_comment_handler))
        .route("/comments/{id}", delete(delete_comment_handler))
        
        // MaterialLabel routes
        .route("/materials/{material_id}/labels", post(create_material_label_handler))
        .route("/materials/{material_id}/labels", get(list_material_labels_handler))
        .route("/materials/{material_id}/labels/{label_id}", delete(delete_material_label_handler))
}
