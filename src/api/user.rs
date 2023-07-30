use std::sync::Arc;

use axum::{
	extract::State,
	routing::{get, post},
	Json, Router,
};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{
	error::{AppError, AppResult},
	models::user::User,
	AppState,
};

pub fn user_router(state: Arc<AppState>) -> Router {
	Router::new()
		.route("/user/create", post(create_user))
		.route("/user/login", post(login_user))
		.route("/user/self", get(get_self))
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
	if username.len() < 3 {
		return Err(AppError::bad_request(
			"Username must be at least 3 characters",
		));
	}
	if password.len() < 8 {
		return Err(AppError::bad_request(
			"Password must be at least 8 characters",
		));
	}
	if !email.contains('@') {
		return Err(AppError::bad_request("Invalid email address"));
	}
	User::create(&state.pool, &username, &email, &password, &false).await?;
	Ok("User created")
}

#[derive(Deserialize)]
pub struct LoginUserRequest {
	pub email: String,
	pub password: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginUserResponse {
	pub id: String,
	pub username: String,
	pub email: String,
	pub is_admin: bool,
	pub token: String,
}

async fn login_user(
	State(state): State<Arc<AppState>>,
	Json(LoginUserRequest { email, password }): Json<LoginUserRequest>,
) -> AppResult<Json<LoginUserResponse>> {
	info!("Logging in user {}...", email);
	let user = User::from_login(&state.pool, &email, &password).await?;
	let token = user.create_token(&state.pool).await?;
	info!("User {} logged in", email);
	Ok(Json(LoginUserResponse {
		id: user.id.to_string(),
		username: user.username,
		email: user.email,
		is_admin: user.is_admin,
		token,
	}))
}

#[derive(Serialize)]
pub struct UserSelfResponse {
	pub id: String,
	pub username: String,
	pub email: String,
	pub is_admin: bool,
}

async fn get_self(user: User) -> AppResult<Json<UserSelfResponse>> {
	Ok(Json(UserSelfResponse {
		id: user.id.to_string(),
		username: user.username,
		email: user.email,
		is_admin: user.is_admin,
	}))
}
