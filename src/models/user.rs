use argon2::{hash_encoded, verify_encoded};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{AppError, AppResult};

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
		if password.len() < 8 {
			return Err(AppError::bad_request(
				"Password must be at least 8 characters",
			));
		}
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

	pub async fn from_login(
		pool: &PgPool,
		username: &String,
		password: &String,
	) -> AppResult<Self> {
		let user = sqlx::query_as!(
			User,
			r#"
SELECT id, username, email, password, is_admin
FROM users
WHERE username = $1
"#,
			username
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
}
