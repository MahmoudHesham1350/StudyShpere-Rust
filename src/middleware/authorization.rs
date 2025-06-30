
use axum::{
    extract::{Request, Path},
    middleware::Next,
    response::Response,
    extract::State,
};
use sqlx::{Pool, Postgres};
use crate::{
    errors::AppError,
    middleware::auth::AuthenticatedUser,
    models
};

pub async fn auth_middleware(
    State(pool): State<Pool<Postgres>>,
    user: AuthenticatedUser,
    Path(group_name): Path<String>,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let group = models::group::Group::find_by_name(&pool, group_name).await?;
    match group {
        Some(g) => {
            if g.owner_id == user.id {
                Ok(next.run(request).await)
            } else {
                Err(AppError::Forbidden)
            }
        },
        None => Err(AppError::NotFound),
        
    }
}