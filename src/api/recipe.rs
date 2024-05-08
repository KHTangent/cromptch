use std::{collections::HashMap, sync::Arc};

use axum::{
	extract::{Path, Query, State},
	routing::{get, post},
	Json, Router,
};
use bigdecimal::FromPrimitive;
use serde::Serialize;
use sqlx::types::BigDecimal;
use tracing::info;
use uuid::Uuid;

use crate::{
	error::{AppError, AppResult},
	models::{
		recipe::{Recipe, RecipeCreation, RecipeListSort, RecipeMetadata},
		user::User,
	},
	AppState,
};

pub fn recipe_router(state: Arc<AppState>) -> Router {
	Router::new()
		.route("/api/recipe/create", post(create_recipe))
		.route("/api/recipe/:id", get(get_recipe))
		.route("/api/recipe/list", get(list_recipes))
		.with_state(state)
}

#[derive(Debug, Serialize)]
struct CreateRecipeResponse {
	id: Uuid,
}

async fn create_recipe(
	State(state): State<Arc<AppState>>,
	user: User,
	Json(recipe): Json<RecipeCreation>,
) -> AppResult<Json<CreateRecipeResponse>> {
	if recipe.name.is_empty() {
		return Err(AppError::bad_request("Title cannot be empty"));
	}
	if recipe.ingredients.len() == 0 {
		return Err(AppError::bad_request(
			"Recipe must have at least one ingredient",
		));
	}
	if recipe.steps.len() == 0 {
		return Err(AppError::bad_request("Recipe must have at least one step"));
	}
	for step in &recipe.steps {
		if step.description.is_empty() {
			return Err(AppError::bad_request("Steps cannot be empty"));
		}
	}
	for ingredient in &recipe.ingredients {
		if ingredient.name.is_empty() {
			return Err(AppError::bad_request("Ingredient name cannot be empty"));
		}
		if ingredient.unit.is_empty() {
			return Err(AppError::bad_request("Ingredient unit cannot be empty"));
		}
		if ingredient.quantity.le(&BigDecimal::from_f32(0.0).unwrap()) {
			return Err(AppError::bad_request(
				"Ingredient quantity cannot be negative",
			));
		}
	}
	let created_recipe = Recipe::create(&state.pool, &user.id, &recipe).await?;
	info!(
		"User {} created recipe {}",
		user.id, created_recipe.metadata.id
	);
	Ok(Json(CreateRecipeResponse {
		id: created_recipe.metadata.id,
	}))
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct GetRecipeResponse {
	recipe: Recipe,
	author: String,
}

async fn get_recipe(
	State(state): State<Arc<AppState>>,
	Path(id): Path<Uuid>,
) -> AppResult<Json<GetRecipeResponse>> {
	let recipe = Recipe::from_uuid(&state.pool, &id).await?;
	let author = User::from_uuid(&state.pool, &recipe.metadata.author).await?;
	Ok(Json(GetRecipeResponse {
		author: author.username,
		recipe,
	}))
}

async fn list_recipes(
	State(state): State<Arc<AppState>>,
	Query(params): Query<HashMap<String, String>>,
) -> AppResult<Json<Vec<RecipeMetadata>>> {
	let limit = params
		.get("limit")
		.map(|s| s.parse::<u64>().unwrap_or(10))
		.unwrap_or(10);
	let sort_order = params
		.get("order")
		.map(|s| match s.as_str() {
			"a-z" => RecipeListSort::NameAscending,
			"z-a" => RecipeListSort::NameDescending,
			"newest" => RecipeListSort::DateDescending,
			"oldest" => RecipeListSort::DateAscending,
			_ => RecipeListSort::DateAscending,
		})
		.unwrap_or(RecipeListSort::DateAscending);
	let recipes = Recipe::list_brief(&state.pool, limit, sort_order).await?;
	Ok(Json(recipes))
}
