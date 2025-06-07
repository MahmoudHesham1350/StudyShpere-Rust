use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct JoinRequest {
    pub group_name: String,
    pub user_id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct JoinRequestWithUser {
    pub group_name: String,
    pub created_at: Option<DateTime<Utc>>,
    pub user_id: Uuid,
    pub user_email: String,
    pub user_name: String,
}

impl JoinRequest {
    pub async fn find_by_group_name(
        pool: &sqlx::Pool<sqlx::Postgres>,
        group_name: &str,
    ) -> Result<Vec<JoinRequestWithUser>, sqlx::Error> {
        let join_requests = sqlx::query_as!(
            JoinRequestWithUser,
            r#"
            SELECT jr.*, u.email as user_email, u.username as user_name
            FROM join_requests jr INNER JOIN users u 
            ON jr.user_id = u.id
            WHERE jr.group_name = $1
            ORDER BY jr.created_at DESC
            "#,
            group_name
        )
        .fetch_all(pool)
        .await?;

        Ok(join_requests)
    }

    pub async fn find_by_group_and_user(
        pool: &sqlx::Pool<sqlx::Postgres>,
        group_name: &str,
        user_id: Uuid,
    ) -> Result<Option<Self>, sqlx::Error> {
        let join_request = sqlx::query_as!(
            JoinRequest,
            r#"
            SELECT group_name, user_id, created_at as "created_at!"
            FROM join_requests
            WHERE group_name = $1 AND user_id = $2
            "#,
            group_name,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(join_request)
    }

    pub async fn create(
        pool: &sqlx::Pool<sqlx::Postgres>,
        new_join_request: JoinRequest,
    ) -> Result<Self, sqlx::Error> {
        let join_request = sqlx::query_as!(
            JoinRequest,
            r#"
            INSERT INTO join_requests (group_name, user_id)
            VALUES ($1, $2)
            RETURNING group_name, user_id, created_at as "created_at!"
            "#,
            new_join_request.group_name,
            new_join_request.user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(join_request)
    }

    pub async fn delete(
        pool: &sqlx::Pool<sqlx::Postgres>,
        group_name: &str,
        user_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM join_requests
            WHERE group_name = $1 AND user_id = $2
            "#,
            group_name,
            user_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
