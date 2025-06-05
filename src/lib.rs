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
use tower_http::trace::TraceLayer;

pub fn create_app(pool: PgPool) -> Router {
    Router::new()
        .nest("/api/auth", routes::auth::auth_routes(pool.clone()))
        .nest("/api/groups", routes::group::group_routes(pool.clone()))
        
        // authenticated routes
        .nest("/api", routes::join_request::join_request_routes()
            .layer(axum::middleware::from_fn_with_state(pool.clone(), middleware::auth_middleware))
        )
        .nest("/api/groups/{group_id}/members", routes::group_member::group_member_routes()
            .layer(axum::middleware::from_fn_with_state(pool.clone(), middleware::auth_middleware))
        )
        .nest("/api", routes::course::course_routes()
            .layer(axum::middleware::from_fn_with_state(pool.clone(), middleware::auth_middleware))
        )
        .nest("/api", routes::material::material_routes()
            .layer(axum::middleware::from_fn_with_state(pool.clone(), middleware::auth_middleware))
        )
        
        .route("/health", get(|| async { "OK" }))
        .layer(TraceLayer::new_for_http())
        .with_state(pool)
}

