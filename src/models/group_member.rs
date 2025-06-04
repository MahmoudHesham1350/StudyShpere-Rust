use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct GroupMember {
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub user_role: String,
    pub joined_at: DateTime<Utc>,
}

impl GroupMember {
    pub async fn create(
        pool: &sqlx::Pool<sqlx::Postgres>,
        new_group_member: GroupMember,
    ) -> Result<Self, sqlx::Error> {
        let group_member = sqlx::query_as!(
            GroupMember,
            r#"
            INSERT INTO group_members (user_id, group_id, user_role, joined_at)
            VALUES ($1, $2, $3, $4)
            RETURNING user_id, group_id, user_role, joined_at as "joined_at!"
            "#,
            new_group_member.user_id,
            new_group_member.group_id,
            new_group_member.user_role,
            new_group_member.joined_at
        )
        .fetch_one(pool)
        .await?;

        Ok(group_member)
    }

    pub async fn find_by_group_id(
        pool: &sqlx::Pool<sqlx::Postgres>,
        group_id: Uuid,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let group_members = sqlx::query_as!(
            GroupMember,
            r#"
            SELECT user_id, group_id, user_role, joined_at as "joined_at!"
            FROM group_members
            WHERE group_id = $1
            ORDER BY joined_at DESC
            "#,
            group_id
        )
        .fetch_all(pool)
        .await?;

        Ok(group_members)
    }

    pub async fn find_by_user_and_group_id(
        pool: &sqlx::Pool<sqlx::Postgres>,
        user_id: Uuid,
        group_id: Uuid,
    ) -> Result<Option<Self>, sqlx::Error> {
        let group_member = sqlx::query_as!(
            GroupMember,
            r#"
            SELECT user_id, group_id, user_role, joined_at as "joined_at!"
            FROM group_members
            WHERE user_id = $1 AND group_id = $2
            "#,
            user_id,
            group_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(group_member)
    }

    pub async fn update(
        pool: &sqlx::Pool<sqlx::Postgres>,
        group_member: GroupMember,
    ) -> Result<Self, sqlx::Error> {
        let updated_group_member = sqlx::query_as!(
            GroupMember,
            r#"
            UPDATE group_members
            SET user_role = $3
            WHERE user_id = $1 AND group_id = $2
            RETURNING user_id, group_id, user_role, joined_at as "joined_at!"
            "#,
            group_member.user_id,
            group_member.group_id,
            group_member.user_role
        )
        .fetch_one(pool)
        .await?;

        Ok(updated_group_member)
    }

    pub async fn delete(
        pool: &sqlx::Pool<sqlx::Postgres>,
        user_id: Uuid,
        group_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM group_members
            WHERE user_id = $1 AND group_id = $2
            "#,
            user_id,
            group_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
