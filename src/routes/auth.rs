// src/routes/auth.rs
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

use crate::{
    handlers::auth::{
        register_handler,
        login_handler,
        logout_handler,
        refresh_token_handler,
        me_handler,
    },
    middleware::auth::auth_middleware,
};

pub fn auth_routes(pool: PgPool) -> Router<PgPool> {
    Router::new()
        // Public routes (no authentication required)
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .route("/refresh", post(refresh_token_handler))
        // Protected routes (authentication required)
        .route("/me", get(me_handler).layer(middleware::from_fn_with_state(pool.clone(),auth_middleware)))

        .route("/logout", post(logout_handler).layer(middleware::from_fn_with_state(pool.clone(),auth_middleware)))

}
