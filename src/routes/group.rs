// src/routes/group.rs
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

use crate::{
    handlers::group::{create_group_handler, list_groups_handler},
    middleware::auth::auth_middleware,
};

pub fn group_routes(pool: PgPool) -> Router<PgPool> {
    Router::new()
    .route("/", post(create_group_handler)).layer(middleware::from_fn_with_state(pool.clone(),auth_middleware))
    .merge(Router::new().route("/", get(list_groups_handler)))
}