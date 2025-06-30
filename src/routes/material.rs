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
};

use crate::handlers::material_label::{
    // MaterialLabel handlers
    create_material_label_handler,
    list_material_labels_handler,
    delete_material_label_handler,
};

pub fn material_routes() -> Router<PgPool> {
    Router::new()
        // Material routes
        .route("/materials", post(create_material_handler))
        .route("/groups/{group_name}/courses/{course_name}/materials", get(list_materials_by_course_handler))
        .route("/materials/{id}", get(get_material_handler))
        .route("/materials/{id}", put(update_material_handler))
        .route("/materials/{id}", delete(delete_material_handler))
        // MaterialLabel routes
        .route("/materials/{material_id}/labels", post(create_material_label_handler))
        .route("/materials/{material_id}/labels", get(list_material_labels_handler))
        .route("/materials/{material_id}/labels/{label_id}", delete(delete_material_label_handler))
}
