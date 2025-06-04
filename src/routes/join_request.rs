// src/routes/join_request.rs
use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

use crate::handlers::join_request::{
    list_join_requests_handler,
    create_join_request_handler,
    respond_to_join_request_handler,
};

pub fn join_request_routes() -> Router<PgPool> {
    Router::new()
        .route("/groups/{group_id}/join-requests", get(list_join_requests_handler).post(create_join_request_handler))
        .route("/groups/{group_id}/join-requests/{join_request_id}", post(respond_to_join_request_handler))
}