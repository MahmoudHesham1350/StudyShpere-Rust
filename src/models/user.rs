use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewUser {
    pub email: String,
    pub username: String,
    pub password_hash: String,
}

impl User {
    pub async fn create(
        pool: &sqlx::Pool<sqlx::Postgres>,
        new_user: NewUser,
    ) -> Result<Self, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (email, username, password_hash)
            VALUES ($1, $2, $3)
            RETURNING id, email, username, password_hash, created_at as "created_at!"
            "#,
            new_user.email,
            new_user.username,
            new_user.password_hash
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_id(
        pool: &sqlx::Pool<sqlx::Postgres>,
        id: Uuid,
    ) -> Result<Option<Self>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, email, username, password_hash, created_at as "created_at!"
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_email(
        pool: &sqlx::Pool<sqlx::Postgres>,
        email: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, email, username, password_hash, created_at as "created_at!"
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_username(
        pool: &sqlx::Pool<sqlx::Postgres>,
        username: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, email, username, password_hash, created_at as "created_at!"
            FROM users
            WHERE username = $1
            "#,
            username
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn update_password(
        pool: &sqlx::Pool<sqlx::Postgres>,
        id: Uuid,
        new_password_hash: String,
    ) -> Result<Self, sqlx::Error> {
        let updated_user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET password_hash = $2
            WHERE id = $1
            RETURNING id, email, username, password_hash, created_at as "created_at!"
            "#,
            id,
            new_password_hash
        )
        .fetch_one(pool)
        .await?;

        Ok(updated_user)
    }

    pub async fn delete(
        pool: &sqlx::Pool<sqlx::Postgres>,
        id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM users
            WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}