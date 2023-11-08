use std::{collections::HashMap, sync::Arc};

use axum::{
	extract::{Path, Query, State},
	routing::{get, post},
	Json, Router,
};
use bigdecimal::ToPrimitive;
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

use crate::{
	error::{AppError, AppResult},
	models::{
		recipe::{Recipe, RecipeMetadata},
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

#[derive(Debug, Deserialize)]
struct CreateRecipeRequest {
	title: String,
	description: String,
	ingredients: Vec<(f32, String, String)>,
	steps: Vec<String>,
}

#[derive(Debug, Serialize)]
struct CreateRecipeResponse {
	id: Uuid,
}

async fn create_recipe(
	State(state): State<Arc<AppState>>,
	user: User,
	Json(CreateRecipeRequest {
		title,
		description,
		ingredients,
		steps,
	}): Json<CreateRecipeRequest>,
) -> AppResult<Json<CreateRecipeResponse>> {
	if title.is_empty() {
		return Err(AppError::bad_request("Title cannot be empty"));
	}
	if ingredients.len() == 0 {
		return Err(AppError::bad_request(
			"Recipe must have at least one ingredient",
		));
	}
	if steps.len() == 0 {
		return Err(AppError::bad_request("Recipe must have at least one step"));
	}
	for step in &steps {
		if step.is_empty() {
			return Err(AppError::bad_request("Steps cannot be empty"));
		}
	}
	for (quantity, unit, name) in &ingredients {
		if name.is_empty() {
			return Err(AppError::bad_request("Ingredient name cannot be empty"));
		}
		if unit.is_empty() {
			return Err(AppError::bad_request("Ingredient unit cannot be empty"));
		}
		if quantity < &0.0 {
			return Err(AppError::bad_request(
				"Ingredient quantity cannot be negative",
			));
		}
	}
	let recipe = Recipe::create(
		&state.pool,
		&title,
		&description,
		&user.id,
		&ingredients,
		&steps,
	)
	.await?;
	info!("User {} created recipe {}", user.id, recipe.id);
	Ok(Json(CreateRecipeResponse { id: recipe.id }))
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct GetRecipeResponse {
	id: Uuid,
	title: String,
	description: String,
	author: String,
	author_id: Uuid,
	ingredients: Vec<(f32, String, String)>,
	steps: Vec<String>,
}

async fn get_recipe(
	State(state): State<Arc<AppState>>,
	Path(id): Path<Uuid>,
) -> AppResult<Json<GetRecipeResponse>> {
	let recipe = Recipe::from_uuid(&state.pool, &id).await?;
	let author = User::from_uuid(&state.pool, &recipe.author).await?;
	Ok(Json(GetRecipeResponse {
		id: recipe.id,
		title: recipe.name,
		description: recipe.description,
		author: author.username,
		author_id: author.id,
		ingredients: recipe
			.ingredients
			.into_iter()
			.map(|i| (i.quantity.to_f32().unwrap_or(0.0), i.unit, i.name))
			.collect(),
		steps: recipe.steps.into_iter().map(|s| s.description).collect(),
	}))
}

async fn list_recipes(
	State(state): State<Arc<AppState>>,
	Query(params): Query<HashMap<String, String>>,
) -> AppResult<Json<Vec<RecipeMetadata>>> {
	let recipes = Recipe::list_brief(&state.pool).await?;
	let limit = params
		.get("limit")
		.map(|s| s.parse::<usize>().unwrap_or(10))
		.unwrap_or(10);
	Ok(Json(recipes.into_iter().take(limit).collect()))
}
