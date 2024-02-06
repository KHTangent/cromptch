use std::str::FromStr;

use serde::Serialize;
use sqlx::{types::BigDecimal, PgPool};
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
	pub ingredients: Vec<RecipeIngredient>,
	pub steps: Vec<RecipeStep>,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum RecipeListSort {
	DateAscending = 1,
	DateDescending = 2,
	NameAscending = 3,
	NameDescending = 4,
}

impl Recipe {
	pub async fn create(
		pool: &PgPool,
		title: &String,
		description: &String,
		author: &Uuid,
		ingredients: &Vec<(f32, String, String)>,
		image_id: Option<Uuid>,
		steps: &Vec<String>,
		step_images: &Vec<Option<Uuid>>,
	) -> AppResult<Recipe> {
		if steps.len() != step_images.len() {
			return Err(AppError::internal("Error creating recipe"));
		}
		let mut tx = pool
			.begin()
			.await
			.map_err(|_| AppError::internal("Internal db error"))?;

		let id = Uuid::new_v4();
		sqlx::query!(
			r#"
			INSERT INTO recipes (id, title, description, author, image_id)
			VALUES ($1, $2, $3, $4, $5)
			"#,
			id,
			title,
			description,
			author,
			image_id
		)
		.execute(&mut *tx)
		.await
		.map_err(|_| AppError::internal("Error creating recipe"))?;

		for (num, (quantity, unit, name)) in ingredients.iter().enumerate() {
			let quantity = BigDecimal::from_str(format!("{:.2}", quantity).as_str())
				.map_err(|_| AppError::internal("Error creating recipe"))?;
			sqlx::query!(
				r#"
				INSERT INTO recipe_ingredients (recipe_id, num, quantity, unit, name)
				VALUES ($1, $2, $3, $4, $5)
				"#,
				id,
				num as i32,
				quantity,
				unit,
				name
			)
			.execute(&mut *tx)
			.await
			.map_err(|_| AppError::internal("Error creating recipe"))?;
		}

		for (num, step) in steps.iter().enumerate() {
			sqlx::query!(
				r#"
				INSERT INTO recipe_steps (recipe_id, num, description, image_id)
				VALUES ($1, $2, $3, $4)
				"#,
				id,
				num as i32,
				step,
				step_images[num]
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
			SELECT id, title, description, author, image_id
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
			SELECT id, title, description, author, image_id
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
}
