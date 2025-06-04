use axum::{
    routing::get,
    Router,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod db;
mod models;
mod handlers;
mod routes;
mod middleware;
mod auth;
mod permissions;
mod errors;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rusty_studyshpere=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = db::init_db_pool()
        .await
        .expect("Failed to initialize database pool");

    let app = Router::new()
        .nest("/api", routes::group::group_routes())
        .nest("/api", routes::join_request::join_request_routes())
        .nest("/api", routes::group_member::group_member_routes())
        .nest("/api", routes::material::material_routes())
        .route("/", get(|| async { "Hello, World!" }))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    tracing::debug!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
