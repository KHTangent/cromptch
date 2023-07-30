use std::sync::Arc;

use argon2::{hash_encoded, verify_encoded};
use axum::{
	async_trait,
	extract::{FromRef, FromRequestParts},
	http::request::Parts,
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use rand::RngCore;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
	error::{AppError, AppResult},
	AppState,
};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User {
	pub id: Uuid,
	pub username: String,
	pub email: String,
	pub password: String,
	pub is_admin: bool,
}

impl User {
	pub async fn create(
		pool: &PgPool,
		username: &String,
		email: &String,
		password: &String,
		is_admin: &bool,
	) -> AppResult<Self> {
		let salt: [u8; 16] = rand::random();
		let hash = hash_encoded(password.as_bytes(), &salt, &argon2::Config::default())
			.map_err(|_| AppError::internal("Failed to hash password"))?;
		let user = sqlx::query_as!(
			User,
			r#"
			INSERT INTO users (username, email, password, is_admin)
			VALUES ($1, $2, $3, $4)
			RETURNING id, username, email, password, is_admin
			"#,
			username,
			email,
			hash,
			is_admin
		)
		.fetch_one(pool)
		.await
		.map_err(|_| AppError::bad_request("Failed to create user"))?;
		Ok(user)
	}

	pub async fn from_uuid(pool: &PgPool, id: &Uuid) -> AppResult<Self> {
		let user = sqlx::query_as!(
			User,
			r#"
			SELECT id, username, email, password, is_admin
			FROM users
			WHERE id = $1
			"#,
			id
		)
		.fetch_one(pool)
		.await
		.map_err(|_| AppError::not_found("User not found"))?;
		Ok(user)
	}

	pub async fn from_token(pool: &PgPool, token: &String) -> AppResult<Self> {
		let user = sqlx::query_as!(
			User,
			r#"
			SELECT u.id, u.username, u.email, u.password, u.is_admin
			FROM users u INNER JOIN user_tokens t ON u.id = t.user_id
			WHERE t.token = $1
			"#,
			token
		)
		.fetch_one(pool)
		.await
		.map_err(|_| AppError::unauthorized("Invalid user token"))?;
		sqlx::query!(
			r#"
			UPDATE user_tokens
			SET last_used = NOW()
			WHERE token = $1
			"#,
			token
		)
		.execute(pool)
		.await
		.map_err(|_| AppError::internal("Failed to update token"))?;
		Ok(user)
	}

	pub async fn from_login(pool: &PgPool, email: &String, password: &String) -> AppResult<Self> {
		let user = sqlx::query_as!(
			User,
			r#"
			SELECT id, username, email, password, is_admin
			FROM users
			WHERE email = $1
			"#,
			email
		)
		.fetch_one(pool)
		.await
		.map_err(|_| AppError::not_found("User not found"))?;
		let valid = verify_encoded(&user.password, password.as_bytes())
			.map_err(|_| AppError::internal("Failed to retrieve user"))?;
		match valid {
			true => Ok(user),
			false => Err(AppError::not_found("User not found")),
		}
	}

	pub async fn create_token(&self, pool: &PgPool) -> AppResult<String> {
		// 48 bytes = 64 characters in base64
		let mut random_bytes: [u8; 48] = [0; 48];
		rand::thread_rng().fill_bytes(&mut random_bytes);
		let token = URL_SAFE_NO_PAD.encode(&random_bytes);
		sqlx::query!(
			r#"
			INSERT INTO user_tokens (token, user_id)
			VALUES ($1, $2)
			"#,
			&token,
			self.id
		)
		.execute(pool)
		.await
		.map_err(|_| AppError::internal("Failed to issue token"))?;
		Ok(token)
	}
}

#[async_trait]
impl<S> FromRequestParts<S> for User
where
	Arc<AppState>: FromRef<S>,
	S: Send + Sync,
{
	type Rejection = AppError;

	async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
		let token = parts
			.headers
			.get("Authorization")
			.ok_or(AppError::unauthorized("Missing authorization header"))?
			.to_str()
			.map_err(|_| AppError::unauthorized("Invalid authorization header"))?
			.trim_start_matches("Bearer ")
			.to_string();
		let state = Arc::from_ref(state);

		let user = User::from_token(&state.pool, &token).await?;
		Ok(user)
	}
}
