use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct GroupMember {
    pub user_id: Uuid,
    pub group_name: String,
    pub user_role: Option<String>,
    pub joined_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct GroupMemberWithUser {
    pub user_id: Uuid,
    pub group_name: String,
    pub user_role: Option<String>,
    pub joined_at: Option<DateTime<Utc>>,
    pub user_email: String,
    pub user_name: String,
}

impl GroupMember {
    pub async fn create(
        pool: &sqlx::Pool<sqlx::Postgres>,
        user_id: Uuid,
        group_name: String,
    ) -> Result<Self, sqlx::Error> {
        let group_member = sqlx::query_as!(
            GroupMember,
            r#"
            INSERT INTO group_members (user_id, group_name)
            VALUES ($1, $2)
            RETURNING user_id, group_name, user_role, joined_at as "joined_at!"
            "#,
            user_id,
            group_name
        )
        .fetch_one(pool)
        .await?;

        Ok(group_member)
    }

    pub async fn find_by_group_name(
        pool: &sqlx::Pool<sqlx::Postgres>,
        group_name: String,
    ) -> Result<Vec<GroupMemberWithUser>, sqlx::Error> {
        let members = sqlx::query_as!(
            GroupMemberWithUser,
            r#"
            SELECT gm.user_id, gm.group_name, gm.user_role, gm.joined_at,
                   u.email as user_email, u.username as user_name
            FROM group_members gm 
            INNER JOIN users u ON gm.user_id = u.id
            WHERE gm.group_name = $1
            ORDER BY gm.joined_at DESC
            "#,
            group_name
        )
        .fetch_all(pool)
        .await?;

        Ok(members)
    }

    pub async fn find_by_user_and_group(
        pool: &sqlx::Pool<sqlx::Postgres>,
        user_id: Uuid,
        group_name: String,
    ) -> Result<Option<Self>, sqlx::Error> {
        let member = sqlx::query_as!(
            GroupMember,
            r#"
            SELECT user_id, group_name, user_role, joined_at as "joined_at!"
            FROM group_members
            WHERE user_id = $1 AND group_name = $2
            "#,
            user_id,
            group_name
        )
        .fetch_optional(pool)
        .await?;

        Ok(member)
    }

    pub async fn update(
        pool: &sqlx::Pool<sqlx::Postgres>,
        user_id: Uuid,
        group_name: String,
        user_role: String,
    ) -> Result<Self, sqlx::Error> {
        let updated_group_member = sqlx::query_as!(
            GroupMember,
            r#"
            UPDATE group_members
            SET user_role = $3
            WHERE user_id = $1 AND group_name = $2
            RETURNING user_id, group_name, user_role, joined_at as "joined_at!"
            "#,
            user_id,
            group_name,
            user_role
        )
        .fetch_one(pool)
        .await?;

        Ok(updated_group_member)
    }

    pub async fn delete(
        pool: &sqlx::Pool<sqlx::Postgres>,
        user_id: Uuid,
        group_name: String,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM group_members
            WHERE user_id = $1 AND group_name = $2
            "#,
            user_id,
            group_name
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
