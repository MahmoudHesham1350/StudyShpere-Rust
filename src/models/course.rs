use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct Course {
    pub group_name: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewCourse {
    pub group_name: String,
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
            INSERT INTO courses (group_name, name, description)
            VALUES ($1, $2, $3)
            RETURNING group_name, name, description, created_at as "created_at!"
            "#,
            new_course.group_name,
            new_course.name,
            new_course.description
        )
        .fetch_one(pool)
        .await?;

        Ok(course)
    }

    pub async fn find_by_group_name(
        pool: &sqlx::Pool<sqlx::Postgres>,
        group_name: &str,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let courses = sqlx::query_as!(
            Course,
            r#"
            SELECT group_name, name, description, created_at as "created_at!"
            FROM courses
            WHERE group_name = $1
            ORDER BY created_at DESC
            "#,
            group_name
        )
        .fetch_all(pool)
        .await?;

        Ok(courses)
    }

    pub async fn find_by_group_and_name(
        pool: &sqlx::Pool<sqlx::Postgres>,
        group_name: &str,
        name: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        let course = sqlx::query_as!(
            Course,
            r#"
            SELECT group_name, name, description, created_at as "created_at!"
            FROM courses
            WHERE group_name = $1 AND name = $2
            "#,
            group_name,
            name
        )
        .fetch_optional(pool)
        .await?;

        Ok(course)
    }

    pub async fn update(
        pool: &sqlx::Pool<sqlx::Postgres>,
        group_name: &str,
        course_name: &str,
        course: Course,
    ) -> Result<Self, sqlx::Error> {
        let updated_course = sqlx::query_as!(
            Course,
            r#"
            UPDATE courses
            SET name = $3, description = $4
            WHERE group_name = $1 AND name = $2
            RETURNING group_name, name, description, created_at as "created_at!"
            "#,
            group_name,
            course_name,
            course.name,
            course.description
        )
        .fetch_one(pool)
        .await?;

        Ok(updated_course)
    }

    pub async fn delete(
        pool: &sqlx::Pool<sqlx::Postgres>,
        group_name: &str,
        name: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM courses
            WHERE group_name = $1 AND name = $2
            "#,
            group_name,
            name
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
