// src/routes/group.rs
use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

use crate::handlers::group::{create_group_handler, list_groups_handler};

pub fn group_routes() -> Router<PgPool> {
    Router::new()
        .route("/groups", post(create_group_handler).get(list_groups_handler))
}