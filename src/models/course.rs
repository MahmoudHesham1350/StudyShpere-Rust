use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct Course {
    pub id: Uuid,
    pub group_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewCourse {
    pub group_id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

impl Course {
    pub async fn create(
        pool: &sqlx::Pool<sqlx::Postgres>,
        new_course: NewCourse,
    ) -> Result<Self, sqlx::Error> {
        let course = sqlx::query_as!(
            Course,
            r#"
            INSERT INTO courses (group_id, name, description)
            VALUES ($1, $2, $3)
            RETURNING id, group_id, name, description, created_at as "created_at!"
            "#,
            new_course.group_id,
            new_course.name,
            new_course.description
        )
        .fetch_one(pool)
        .await?;

        Ok(course)
    }

    pub async fn find_by_group_id(
        pool: &sqlx::Pool<sqlx::Postgres>,
        group_id: Uuid,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let courses = sqlx::query_as!(
            Course,
            r#"
            SELECT id, group_id, name, description, created_at as "created_at!"
            FROM courses
            WHERE group_id = $1
            ORDER BY created_at DESC
            "#,
            group_id
        )
        .fetch_all(pool)
        .await?;

        Ok(courses)
    }

    pub async fn find_by_id(
        pool: &sqlx::Pool<sqlx::Postgres>,
        id: Uuid,
    ) -> Result<Option<Self>, sqlx::Error> {
        let course = sqlx::query_as!(
            Course,
            r#"
            SELECT id, group_id, name, description, created_at as "created_at!"
            FROM courses
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(course)
    }

    pub async fn update(
        pool: &sqlx::Pool<sqlx::Postgres>,
        course: Course,
    ) -> Result<Self, sqlx::Error> {
        let updated_course = sqlx::query_as!(
            Course,
            r#"
            UPDATE courses
            SET name = $2, description = $3
            WHERE id = $1
            RETURNING id, group_id, name, description, created_at as "created_at!"
            "#,
            course.id,
            course.name,
            course.description
        )
        .fetch_one(pool)
        .await?;

        Ok(updated_course)
    }

    pub async fn delete(
        pool: &sqlx::Pool<sqlx::Postgres>,
        id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM courses
            WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
