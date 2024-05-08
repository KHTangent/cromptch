use chrono::naive::serde::ts_seconds;
use serde::Serialize;
use sqlx::{
	types::{chrono::NaiveDateTime, BigDecimal},
	PgPool,
};
use uuid::Uuid;

use crate::error::{AppError, AppResult};

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeMetadata {
	pub id: Uuid,
	pub title: String,
	pub description: String,
	pub author: Uuid,
	pub image_id: Option<Uuid>,
	pub time_estimate_active: Option<BigDecimal>,
	pub time_estimate_total: Option<BigDecimal>,
	pub source_url: Option<String>,
	#[serde(with = "ts_seconds")]
	pub created_at: NaiveDateTime,
	#[serde(with = "ts_seconds")]
	pub edited_at: NaiveDateTime,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct RecipeIngredient {
	pub quantity: BigDecimal,
	pub unit: String,
	pub name: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct RecipeStep {
	pub description: String,
	pub image_id: Option<Uuid>,
}

#[derive(Debug, Clone)]
pub struct Recipe {
	pub id: Uuid,
	pub name: String,
	pub description: String,
	pub author: Uuid,
	pub image_id: Option<Uuid>,
	pub time_estimate_active: Option<BigDecimal>,
	pub time_estimate_total: Option<BigDecimal>,
	pub created_at: NaiveDateTime,
	pub edited_at: NaiveDateTime,
	pub source_url: Option<String>,
	pub ingredients: Vec<RecipeIngredient>,
	pub steps: Vec<RecipeStep>,
}

#[derive(Debug, Clone)]
pub struct RecipeCreation {
	pub name: String,
	pub description: String,
	pub author: Uuid,
	pub image_id: Option<Uuid>,
	pub time_estimate_active: Option<BigDecimal>,
	pub time_estimate_total: Option<BigDecimal>,
	pub source_url: Option<String>,
	pub ingredients: Vec<RecipeIngredient>,
	pub steps: Vec<RecipeStep>,
}

#[derive(Clone, Copy)]
pub enum RecipeListSort {
	DateAscending = 1,
	DateDescending = 2,
	NameAscending = 3,
	NameDescending = 4,
}

impl Recipe {
	pub async fn create(pool: &PgPool, data: &RecipeCreation) -> AppResult<Recipe> {
		let mut tx = pool
			.begin()
			.await
			.map_err(|_| AppError::internal("Internal db error"))?;

		let id = Uuid::new_v4();
		sqlx::query!(
			r#"
			INSERT INTO recipes (id, title, description, author, image_id, source_url, time_estimate_active, time_estimate_total)
			VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
			"#,
			id,
			data.name,
			data.description,
			data.author,
			data.image_id,
            data.source_url,
			data.time_estimate_active,
			data.time_estimate_total,
		)
		.execute(&mut *tx)
		.await
		.map_err(|_| AppError::internal("Error creating recipe"))?;

		for (num, ingredient) in data.ingredients.iter().enumerate() {
			sqlx::query!(
				r#"
				INSERT INTO recipe_ingredients (recipe_id, num, quantity, unit, name)
				VALUES ($1, $2, $3, $4, $5)
				"#,
				id,
				num as i32,
				ingredient.quantity,
				ingredient.unit,
				ingredient.name
			)
			.execute(&mut *tx)
			.await
			.map_err(|_| AppError::internal("Error creating recipe"))?;
		}

		for (num, step) in data.steps.iter().enumerate() {
			sqlx::query!(
				r#"
				INSERT INTO recipe_steps (recipe_id, num, description, image_id)
				VALUES ($1, $2, $3, $4)
				"#,
				id,
				num as i32,
				step.description,
				step.image_id,
			)
			.execute(&mut *tx)
			.await
			.map_err(|_| AppError::internal("Error creating recipe"))?;
		}

		tx.commit()
			.await
			.map_err(|_| AppError::internal("Internal db error"))?;

		Ok(Recipe::from_uuid(pool, &id).await?)
	}

	pub async fn from_uuid(pool: &PgPool, id: &Uuid) -> AppResult<Recipe> {
		let metadata = sqlx::query_as!(
			RecipeMetadata,
			r#"
			SELECT id, title, description, author, image_id, time_estimate_active, time_estimate_total, source_url, created_at, edited_at
			FROM recipes
			WHERE id = $1
			"#,
			id
		)
		.fetch_one(pool)
		.await
		.map_err(|_| AppError::not_found("Recipe not found"))?;

		let ingredients = sqlx::query_as!(
			RecipeIngredient,
			r#"
			SELECT quantity, unit, name
			FROM recipe_ingredients
			WHERE recipe_id = $1
			ORDER BY num
			"#,
			id
		)
		.fetch_all(pool)
		.await
		.map_err(|_| AppError::not_found("Recipe not found"))?;

		let steps = sqlx::query_as!(
			RecipeStep,
			r#"
			SELECT description, image_id
			FROM recipe_steps
			WHERE recipe_id = $1
			ORDER BY num
			"#,
			id
		)
		.fetch_all(pool)
		.await
		.map_err(|_| AppError::not_found("Recipe not found"))?;

		Ok(Recipe {
			id: metadata.id,
			name: metadata.title,
			description: metadata.description,
			author: metadata.author,
			image_id: metadata.image_id,
			steps,
			ingredients,
			time_estimate_active: metadata.time_estimate_active,
			time_estimate_total: metadata.time_estimate_total,
			source_url: metadata.source_url,
			created_at: metadata.created_at,
			edited_at: metadata.edited_at,
		})
	}

	pub async fn list_brief(
		pool: &PgPool,
		max_count: u64,
		ordering: RecipeListSort,
	) -> AppResult<Vec<RecipeMetadata>> {
		let recipes = sqlx::query_as!(
			RecipeMetadata,
			r#"
			SELECT id, title, description, author, image_id, time_estimate_active, time_estimate_total, source_url, created_at, edited_at
			FROM recipes
			ORDER BY
				CASE WHEN $2 = 1 THEN created_at END ASC,
				CASE WHEN $2 = 2 THEN created_at END DESC,
				CASE WHEN $2 = 3 THEN title END ASC,
				CASE WHEN $2 = 4 THEN title END DESC
			LIMIT $1
			"#,
			max_count as i64,
			ordering as i64,
		)
		.fetch_all(pool)
		.await
		.map_err(|_| AppError::internal("Error fetching recipes"))?;
		Ok(recipes)
	}

	pub async fn delete(&self, pool: &PgPool) -> AppResult<()> {
		sqlx::query!(
			r#"
			DELETE FROM recipes
			WHERE id = $1
			"#,
			self.id
		)
		.execute(pool)
		.await
		.map_err(|_| AppError::internal("Deletion failed"))?;
		Ok(())
	}
}
