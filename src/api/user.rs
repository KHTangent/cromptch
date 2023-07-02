use std::sync::Arc;

use axum::{extract::State, routing::post, Json, Router};
use serde::Deserialize;
use tracing::info;

use crate::{error::AppResult, models::user::User, AppState};

pub fn user_router(state: Arc<AppState>) -> Router {
	Router::new()
		.route("/user", post(create_user))
		.with_state(state)
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
	pub username: String,
	pub email: String,
	pub password: String,
}

async fn create_user(
	State(state): State<Arc<AppState>>,
	Json(CreateUserRequest {
		username,
		email,
		password,
	}): Json<CreateUserRequest>,
) -> AppResult<&'static str> {
	info!("Creating user {}...", username);
	User::create(&state.pool, &username, &email, &password, &false).await?;
	Ok("User created")
}
