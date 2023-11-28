use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{AppError, AppResult};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Image {
	pub id: Uuid,
	pub delete_token: Uuid,
}

impl Image {
	pub async fn create(pool: &PgPool, id: &Uuid, delete_token: &Uuid) -> AppResult<Self> {
		sqlx::query!(
			r#"
                INSERT INTO images (id, delete_token)
                VALUES ($1, $2)
            "#,
			id,
			delete_token,
		)
		.execute(pool)
		.await
		.map_err(|_| AppError::internal("Failed to create image entry"))?;

		Ok(Self {
			id: id.clone(),
			delete_token: delete_token.clone(),
		})
	}
}
