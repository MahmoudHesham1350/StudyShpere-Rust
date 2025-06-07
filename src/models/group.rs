use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct Group {
    pub owner_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub join_type: Option<String>,
    pub post_permission: Option<String>,
    pub edit_permissions: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewGroup {
    pub name: String,
    pub description: Option<String>,
    pub owner_id: Uuid,
    pub join_type: String,
}



impl Group {
    pub async fn create(
        pool: &sqlx::Pool<sqlx::Postgres>,
        new_group: NewGroup,
    ) -> Result<Self, sqlx::Error> {
        let group = sqlx::query_as!(
            Group,
            r#"
            INSERT INTO groups (owner_id, name, description, join_type)
            VALUES ($1, $2, $3, $4)
            RETURNING owner_id, name, description, join_type, post_permission, edit_permissions, created_at as "created_at!"
            "#,
            new_group.owner_id,
            new_group.name,
            new_group.description,
            new_group.join_type
        )
        .fetch_one(pool)
        .await?;

        Ok(group)
    }

    pub async fn find_all(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<Vec<Self>, sqlx::Error> {
        let groups = sqlx::query_as!(
            Group,
            r#"
            SELECT owner_id, name, description, join_type, post_permission, edit_permissions, created_at as "created_at!"
            FROM groups
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(groups)
    }

    pub async fn find_by_name(
        pool: &sqlx::Pool<sqlx::Postgres>,
        name: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        let group = sqlx::query_as!(
            Group,
            r#"
            SELECT owner_id, name, description, join_type, post_permission, edit_permissions, created_at as "created_at!"
            FROM groups
            WHERE name = $1
            "#,
            name
        )
        .fetch_optional(pool)
        .await?;

        Ok(group)
    }
}
