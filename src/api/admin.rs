use std::sync::Arc;

use axum::{
	extract::{Path, State},
	routing::{delete, get},
	Json, Router,
};
use serde::Serialize;
use tracing::info;
use uuid::Uuid;

use crate::{
	error::AppResult,
	models::{admin::Admin, recipe::Recipe, user::User},
	AppState,
};

pub fn admin_router(state: Arc<AppState>) -> Router {
	Router::new()
		.route("/api/admin/users", get(get_users))
		.route("/api/admin/recipe/:id", delete(delete_recipe))
		.with_state(state)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserViewResponseEntry {
	pub id: String,
	pub username: String,
	pub email: String,
	pub is_admin: bool,
}

async fn get_users(
	State(state): State<Arc<AppState>>,
	_: Admin,
) -> AppResult<Json<Vec<UserViewResponseEntry>>> {
	let users = User::get_all(&state.pool).await?;
	Ok(Json(
		users
			.into_iter()
			.map(|u| UserViewResponseEntry {
				id: u.id.to_string(),
				username: u.username,
				email: u.email,
				is_admin: u.is_admin,
			})
			.collect(),
	))
}

async fn delete_recipe(
	State(state): State<Arc<AppState>>,
	admin: Admin,
	Path(id): Path<Uuid>,
) -> AppResult<()> {
	let recipe = Recipe::from_uuid(&state.pool, &id).await?;
	info!("User {} deleted recipe {}", admin.user.id, recipe.id);
	recipe.delete(&state.pool).await?;
	Ok(())
}
