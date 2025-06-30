use axum::{
    routing::{get, post, put, delete},
    Router,
};
use sqlx::PgPool;

use crate::handlers::comment::{
// Comment handlers
create_comment_handler,
list_comments_handler,
get_comment_handler,
update_comment_handler,
delete_comment_handler,
};


pub fn comment_routes() -> Router<PgPool> {
    Router::new()
        // Comment routes
        .route("/materials/{material_id}/comments", post(create_comment_handler))
        .route("/materials/{material_id}/comments", get(list_comments_handler))
        .route("/materials/{material_id}/comments/{id}", get(get_comment_handler))
        .route("/materials/{material_id}/comments/{id}", put(update_comment_handler))
        .route("/materials/{material_id}/comments/{id}", delete(delete_comment_handler))
        
}
