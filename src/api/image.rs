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
	error::AppResult,
	external::image::{get_image_bytes, get_thumbnail_bytes},
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
	let image_bytes = get_image_bytes(&state.secrets.pictrs_url, &id).await?;
	Ok((
		http::StatusCode::OK,
		[(http::header::CONTENT_TYPE, "image/webp")],
		image_bytes,
	))
}

async fn get_thumbnail(
	State(state): State<Arc<AppState>>,
	Path(id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
	let image_bytes = get_thumbnail_bytes(&state.secrets.pictrs_url, &id).await?;
	Ok((
		http::StatusCode::OK,
		[(http::header::CONTENT_TYPE, "image/webp")],
		image_bytes,
	))
}
