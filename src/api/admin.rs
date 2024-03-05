use std::sync::Arc;

use axum::{extract::State, routing::get, Json, Router};
use serde::Serialize;

use crate::{
	error::AppResult,
	models::{admin::Admin, user::User},
	AppState,
};

pub fn admin_router(state: Arc<AppState>) -> Router {
	Router::new()
		.route("/api/admin/users", get(get_users))
		.with_state(state)
}

#[derive(Serialize)]
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
