use uuid::Uuid;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct MaterialLabel {
    pub material_id: Uuid,
    pub group_name: String,
    pub label_name: String,
    pub number: i32,
}

impl MaterialLabel {
    pub async fn create(
        pool: &sqlx::Pool<sqlx::Postgres>,
        material: MaterialLabel,
    ) -> Result<Self, sqlx::Error> {
        let material_label = sqlx::query_as!(
            MaterialLabel,
            r#"
            INSERT INTO material_labels (material_id, group_name, label_name, number)
            VALUES ($1, $2, $3, $4)
            RETURNING material_id, group_name, label_name, number
            "#,
            material.material_id,
            material.group_name,
            material.label_name,
            material.number
        )
        .fetch_one(pool)
        .await?;

        Ok(material_label)
    }

    pub async fn find_by_material_id(
        pool: &sqlx::Pool<sqlx::Postgres>,
        material_id: Uuid,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let material_labels = sqlx::query_as!(
            MaterialLabel,
            r#"
            SELECT *
            FROM material_labels
            WHERE material_id = $1
            ORDER BY number ASC
            "#,
            material_id
        )
        .fetch_all(pool)
        .await?;

        Ok(material_labels)
    }

    pub async fn delete(
        pool: &sqlx::Pool<sqlx::Postgres>,
        material_id: Uuid,
        label_name: String,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM material_labels
            WHERE material_id = $1 AND label_name = $2
            "#,
            material_id,
            label_name
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
