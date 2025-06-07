use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct Material {
    pub id: Uuid,
    pub group_name: String,
    pub course_name: String,
    pub title: String,
    pub file: Option<String>,
    pub url: Option<String>,
    #[serde(rename = "type")]
    #[sqlx(rename = "type")]
    pub material_type: String, // Renamed to avoid Rust keyword conflict
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub creator: Uuid,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewMaterial {
    pub group_name: String,
    pub course_name: String,
    pub title: String,
    pub file: Option<String>,
    pub url: Option<String>,
    pub material_type: String,
    pub creator: Uuid,
}

impl Material {
    pub async fn create(
        pool: &sqlx::Pool<sqlx::Postgres>,
        new_material: NewMaterial,
    ) -> Result<Self, sqlx::Error> {
        let material = sqlx::query_as!(
            Material,
            r#"
            INSERT INTO materials (group_name, course_name, title, file, url, type, creator)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, group_name, course_name, title, file, url, type as "material_type", created_at as "created_at!", updated_at as "updated_at!", creator
            "#,
            new_material.group_name,
            new_material.course_name,
            new_material.title,
            new_material.file,
            new_material.url,
            new_material.material_type,
            new_material.creator
        )
        .fetch_one(pool)
        .await?;

        Ok(material)
    }

    pub async fn find_by_course(
        pool: &sqlx::Pool<sqlx::Postgres>,
        group_name: &str,
        course_name: &str,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let materials = sqlx::query_as!(
            Material,
            r#"
            SELECT id, group_name, course_name, title, file, url, type as "material_type", created_at as "created_at!", updated_at as "updated_at!", creator
            FROM materials
            WHERE group_name = $1 AND course_name = $2
            ORDER BY created_at DESC
            "#,
            group_name,
            course_name
        )
        .fetch_all(pool)
        .await?;

        Ok(materials)
    }

    pub async fn find_by_id(
        pool: &sqlx::Pool<sqlx::Postgres>,
        id: Uuid,
    ) -> Result<Option<Self>, sqlx::Error> {
        let material = sqlx::query_as!(
            Material,
            r#"
            SELECT id, group_name, course_name, title, file, url, type as "material_type", created_at as "created_at!", updated_at as "updated_at!", creator
            FROM materials
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(material)
    }

    pub async fn update(
        pool: &sqlx::Pool<sqlx::Postgres>,
        id: Uuid,
        title: String,
        file: Option<String>,
        url: Option<String>,
        material_type: String,
    ) -> Result<Self, sqlx::Error> {
        let updated_material = sqlx::query_as!(
            Material,
            r#"
            UPDATE materials
            SET title = $2, file = $3, url = $4, type = $5, updated_at = NOW()
            WHERE id = $1
            RETURNING id, group_name, course_name, title, file, url, type as "material_type", created_at as "created_at!", updated_at as "updated_at!", creator
            "#,
            id,
            title,
            file,
            url,
            material_type
        )
        .fetch_one(pool)
        .await?;

        Ok(updated_material)
    }

    pub async fn delete(
        pool: &sqlx::Pool<sqlx::Postgres>,
        id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM materials
            WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
