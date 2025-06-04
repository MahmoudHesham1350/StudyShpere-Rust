use uuid::Uuid;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct MaterialLabel {
    pub material_id: Uuid,
    pub label_id: Uuid,
    pub number: i32,
}

impl MaterialLabel {
    pub async fn create(
        pool: &sqlx::Pool<sqlx::Postgres>,
        material_id: Uuid,
        label_id: Uuid,
        number: i32,
    ) -> Result<Self, sqlx::Error> {
        let material_label = sqlx::query_as!(
            MaterialLabel,
            r#"
            INSERT INTO material_labels (material_id, label_id, number)
            VALUES ($1, $2, $3)
            RETURNING material_id, label_id, number
            "#,
            material_id,
            label_id,
            number
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
            SELECT material_id, label_id, number
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
        label_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM material_labels
            WHERE material_id = $1 AND label_id = $2
            "#,
            material_id,
            label_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
