#![allow(dead_code)]
use std::fmt;

use axum::{
	http::StatusCode,
	response::{IntoResponse, Response},
};

pub type AppResult<T> = Result<T, AppError>;

#[derive(fmt::Debug)]
pub struct AppError {
	kind: AppErrorKind,
	message: String,
}

#[derive(fmt::Debug)]
enum AppErrorKind {
	BadRequest,
	Unauthorized,
	Forbidden,
	NotFound,
	InternalServerError,
}

impl fmt::Display for AppError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self.kind {
			AppErrorKind::BadRequest => write!(f, "Bad Request: {}", self.message),
			AppErrorKind::Unauthorized => write!(f, "Unauthorized: {}", self.message),
			AppErrorKind::Forbidden => write!(f, "Forbidden: {}", self.message),
			AppErrorKind::NotFound => write!(f, "Not Found: {}", self.message),
			AppErrorKind::InternalServerError => {
				write!(f, "Internal Server Error: {}", self.message)
			}
		}
	}
}

impl Into<StatusCode> for AppErrorKind {
	fn into(self) -> StatusCode {
		match self {
			AppErrorKind::BadRequest => StatusCode::BAD_REQUEST,
			AppErrorKind::Unauthorized => StatusCode::UNAUTHORIZED,
			AppErrorKind::Forbidden => StatusCode::FORBIDDEN,
			AppErrorKind::NotFound => StatusCode::NOT_FOUND,
			AppErrorKind::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
		}
	}
}

impl IntoResponse for AppError {
	fn into_response(self) -> Response {
		let status_code: StatusCode = self.kind.into();
		(status_code, self.message).into_response()
	}
}

impl AppError {
	pub fn bad_request(e: impl ToString) -> Self {
		Self {
			kind: AppErrorKind::BadRequest,
			message: e.to_string(),
		}
	}

	pub fn unauthorized(e: impl ToString) -> Self {
		Self {
			kind: AppErrorKind::Unauthorized,
			message: e.to_string(),
		}
	}

	pub fn forbidden(e: impl ToString) -> Self {
		Self {
			kind: AppErrorKind::Forbidden,
			message: e.to_string(),
		}
	}

	pub fn not_found(e: impl ToString) -> Self {
		Self {
			kind: AppErrorKind::NotFound,
			message: e.to_string(),
		}
	}

	pub fn internal(e: impl ToString) -> Self {
		Self {
			kind: AppErrorKind::InternalServerError,
			message: e.to_string(),
		}
	}
}
