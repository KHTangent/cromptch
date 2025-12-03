use std::sync::Arc;

use crate::{error::AppError, AppState};

use super::user::User;
use axum::{
	async_trait,
	extract::{FromRef, FromRequestParts},
	http::request::Parts,
};

#[derive(Debug, Clone)]
pub struct Admin {
	pub user: User,
}

#[async_trait]
impl<S> FromRequestParts<S> for Admin
where
	Arc<AppState>: FromRef<S>,
	S: Send + Sync,
{
	type Rejection = AppError;

	async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
		let user = User::from_request_parts(parts, state).await?;
		user.is_admin
			.then_some(Admin { user })
			.ok_or_else(|| AppError::unauthorized("user is not an admin"))
	}
}
