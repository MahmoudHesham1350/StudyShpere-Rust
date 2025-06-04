use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct Group {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub join_type: String,
    pub post_permission: String,
    pub edit_permissions: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewGroup {
    pub name: String,
    pub description: Option<String>,
    pub owner_id: Uuid,
}

impl Group {
    pub async fn create(
        pool: &sqlx::Pool<sqlx::Postgres>,
        new_group: NewGroup,
    ) -> Result<Self, sqlx::Error> {
        let group = sqlx::query_as!(
            Group,
            r#"
            INSERT INTO groups (owner_id, name, description, join_type, post_permission, edit_permissions)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, owner_id, name, description, join_type, post_permission, edit_permissions, created_at as "created_at!"
            "#,
            new_group.owner_id,
            new_group.name,
            new_group.description,
            "public", // Default join_type
            "all_members", // Default post_permission
            "owner_only", // Default edit_permissions
        )
        .fetch_one(pool)
        .await?;

        Ok(group)
    }

    pub async fn find_all(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<Vec<Self>, sqlx::Error> {
        let groups = sqlx::query_as!(
            Group,
            r#"
            SELECT id, owner_id, name, description, join_type, post_permission, edit_permissions, created_at as "created_at!"
            FROM groups
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(groups)
    }

    pub async fn find_by_id(
        pool: &sqlx::Pool<sqlx::Postgres>,
        id: Uuid,
    ) -> Result<Option<Self>, sqlx::Error> {
        let group = sqlx::query_as!(
            Group,
            r#"
            SELECT id, owner_id, name, description, join_type, post_permission, edit_permissions, created_at as "created_at!"
            FROM groups
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(group)
    }
}
