use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct JoinRequest {
    pub id: Uuid,
    pub group_id: Uuid,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
}

impl JoinRequest {
    pub async fn find_by_group_id(
        pool: &sqlx::Pool<sqlx::Postgres>,
        group_id: Uuid,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let join_requests = sqlx::query_as!(
            JoinRequest,
            r#"
            SELECT id, group_id, user_id, created_at as "created_at!"
            FROM join_requests
            WHERE group_id = $1
            ORDER BY created_at DESC
            "#,
            group_id
        )
        .fetch_all(pool)
        .await?;

        Ok(join_requests)
    }

    pub async fn find_by_id(
        pool: &sqlx::Pool<sqlx::Postgres>,
        id: Uuid,
    ) -> Result<Option<Self>, sqlx::Error> {
        let join_request = sqlx::query_as!(
            JoinRequest,
            r#"
            SELECT id, group_id, user_id, created_at as "created_at!"
            FROM join_requests
            WHERE id = $1
            "#,
            id
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
            INSERT INTO join_requests (id, group_id, user_id, created_at)
            VALUES ($1, $2, $3, $4)
            RETURNING id, group_id, user_id, created_at as "created_at!"
            "#,
            new_join_request.id,
            new_join_request.group_id,
            new_join_request.user_id,
            new_join_request.created_at
        )
        .fetch_one(pool)
        .await?;

        Ok(join_request)
    }

    pub async fn delete(
        pool: &sqlx::Pool<sqlx::Postgres>,
        id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM join_requests
            WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
