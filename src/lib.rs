// src/lib.rs
pub mod db;
pub mod models;
pub mod handlers;
pub mod routes;
pub mod middleware;
pub mod auth;
pub mod permissions;
pub mod errors;
pub mod utils;

use axum::{routing::get, Router};
use sqlx::PgPool;

pub fn create_app(pool: PgPool) -> Router {
    Router::new()
        .nest("/api", routes::group::group_routes())
        .nest("/api", routes::join_request::join_request_routes())
        .nest("/api", routes::group_member::group_member_routes())
        .nest("/api", routes::course::course_routes())
        .nest("/api", routes::material::material_routes())
        .route("/health", get(|| async { "OK" }))
        .route("/", get(|| async { "Hello, World!" }))
        .with_state(pool)
}
