use sqlx::PgPool;

use crate::{error::AppError, models::Workspace};

impl Workspace {
    pub async fn create(name: &str, user_id: u64, pool: &PgPool) -> Result<Self, AppError> {
        let ws = sqlx::query_as(
            r#"
            INSERT INTO workspace (name, owner_id) 
            VALUES ($1, $2) 
            RETURNING id, name, owner_id, create_at
            "#,
        )
        .bind(name)
        .bind(user_id as i64)
        .fetch_one(pool)
        .await?;
        Ok(ws)
    }

    pub async fn update_owner(id: u64, owner_id: u64, pool: &PgPool) -> Result<Self, AppError> {
        let ws = sqlx::query_as(
            r#"
            UPDATE workspace SET owner_id = $2 WHERE id = $1 and (SELECT ws_id FROM users WHERE id = $2) = $1
            RETURNING id, name, owner_id, creat_at
            "#,
        )
        .bind(id as i64)
        .bind(owner_id as i64)
        .fetch_one(pool)
        .await?;

        Ok(ws)
    }
}
