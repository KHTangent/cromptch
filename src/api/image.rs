use std::sync::Arc;

use axum::{
	extract::{Path, State},
	http,
	response::IntoResponse,
	routing::get,
	Router,
};
use uuid::Uuid;

use crate::{
	error::{AppError, AppResult},
	AppState,
};

pub fn image_router(state: Arc<AppState>) -> Router {
	if state.secrets.pictrs_url.is_empty() {
		return Router::new();
	}
	Router::new()
		.route("/api/image/:id", get(get_image))
		.route("/api/image/thumbnail/:id", get(get_thumbnail))
		.with_state(state)
}

async fn get_image(
	State(state): State<Arc<AppState>>,
	Path(id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
	let full_picture_url = format!("{}/image/original/{}.webp", state.secrets.pictrs_url, id);
	println!("Fetching image from {}", full_picture_url);
	let response = reqwest::get(&full_picture_url)
		.await
		.map_err(|_| AppError::not_found("Image not found"))?;
	let bytes = response
		.bytes()
		.await
		.map_err(|_| AppError::internal("Error fetching image"))?;
	Ok((
		http::StatusCode::OK,
		[(http::header::CONTENT_TYPE, "image/webp")],
		bytes,
	))
}

async fn get_thumbnail(
	State(state): State<Arc<AppState>>,
	Path(id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
	let thumbnail_url = format!(
		"{}/image/process.webp?thumbnail=200&src={}.webp",
		state.secrets.pictrs_url, id
	);
	println!("Fetching image from {}", thumbnail_url);
	let response = reqwest::get(&thumbnail_url)
		.await
		.map_err(|_| AppError::not_found("Image not found"))?;
	let bytes = response
		.bytes()
		.await
		.map_err(|_| AppError::internal("Error fetching image"))?;
	Ok((
		http::StatusCode::OK,
		[(http::header::CONTENT_TYPE, "image/webp")],
		bytes,
	))
}
