use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct Comment {
    pub id: i32,
    pub material_id: Uuid,
    pub user_id: Option<Uuid>,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewComment {
    pub material_id: Uuid,
    pub user_id: Option<Uuid>,
    pub content: String,
}

impl Comment {
    pub async fn create(
        pool: &sqlx::Pool<sqlx::Postgres>,
        new_comment: NewComment,
    ) -> Result<Self, sqlx::Error> {
        let comment = sqlx::query_as!(
            Comment,
            r#"
            INSERT INTO comments (material_id, user_id, content)
            VALUES ($1, $2, $3)
            RETURNING id, material_id, user_id, content, created_at as "created_at!"
            "#,
            new_comment.material_id,
            new_comment.user_id,
            new_comment.content
        )
        .fetch_one(pool)
        .await?;

        Ok(comment)
    }

    pub async fn find_by_material_id(
        pool: &sqlx::Pool<sqlx::Postgres>,
        material_id: Uuid,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let comments = sqlx::query_as!(
            Comment,
            r#"
            SELECT id, material_id, user_id, content, created_at as "created_at!"
            FROM comments
            WHERE material_id = $1
            ORDER BY created_at ASC
            "#,
            material_id
        )
        .fetch_all(pool)
        .await?;

        Ok(comments)
    }

    pub async fn find_by_id(
        pool: &sqlx::Pool<sqlx::Postgres>,
        material_id: Uuid,
        id: i32,
    ) -> Result<Option<Self>, sqlx::Error> {
        let comment = sqlx::query_as!(
            Comment,
            r#"
            SELECT id, material_id, user_id, content, created_at as "created_at!"
            FROM comments
            WHERE material_id = $1 AND id = $2
            "#,
            material_id,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(comment)
    }

    pub async fn update(
        pool: &sqlx::Pool<sqlx::Postgres>,
        material_id: Uuid,
        id: i32,
        content: String,
    ) -> Result<Self, sqlx::Error> {
        let updated_comment = sqlx::query_as!(
            Comment,
            r#"
            UPDATE comments
            SET content = $3
            WHERE material_id = $1 AND id = $2
            RETURNING id, material_id, user_id, content, created_at as "created_at!"
            "#,
            material_id,
            id,
            content
        )
        .fetch_one(pool)
        .await?;

        Ok(updated_comment)
    }

    pub async fn delete(
        pool: &sqlx::Pool<sqlx::Postgres>,
        material_id: Uuid,
        id: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM comments
            WHERE material_id = $1 AND id = $2
            "#,
            material_id,
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
