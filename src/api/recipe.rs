use std::sync::Arc;

use axum::{
	extract::State,
	routing::{get, post},
	Json, Router,
};
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

use crate::{
	error::AppResult,
	models::{recipe::Recipe, user::User},
	AppState,
};

pub fn recipe_router(state: Arc<AppState>) -> Router {
	Router::new()
		.route("/recipe/create", post(create_recipe))
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
	let recipe = Recipe::create(
		&state.pool,
		&title,
		&description,
		&user.id,
		&ingredients,
		&steps,
	)
	.await?;
	Ok(Json(CreateRecipeResponse { id: recipe.id }))
}
