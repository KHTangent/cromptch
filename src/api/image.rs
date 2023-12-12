use std::sync::Arc;

use axum::{
	extract::{Multipart, Path, State},
	http,
	response::IntoResponse,
	routing::{get, post},
	Json, Router,
};
use serde::Serialize;
use uuid::Uuid;

use crate::{
	error::{AppError, AppResult},
	external::{
		self,
		image::{get_image_bytes, get_thumbnail_bytes},
	},
	models, AppState,
};

pub fn image_router(state: Arc<AppState>) -> Router {
	if state.secrets.pictrs_url.is_empty() {
		return Router::new();
	}
	Router::new()
		.route("/api/image/:id", get(get_image))
		.route("/api/image/thumbnail/:id", get(get_thumbnail))
		.route("/api/image", post(upload_image))
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

#[derive(Debug, Serialize)]
struct UploadImageResponse {
	id: Uuid,
}

async fn upload_image(
	State(state): State<Arc<AppState>>,
	mut multipart: Multipart,
) -> AppResult<Json<UploadImageResponse>> {
	let field = multipart
		.next_field()
		.await
		.map_err(|_| AppError::bad_request("Missing file"))?
		.ok_or(AppError::internal("Failed to parse form object"))?;
	let form_name = field
		.name()
		.ok_or(AppError::bad_request("Bad form field"))?;
	if form_name != "file" {
		return Err(AppError::bad_request("Invalid form body"));
	}
	let file_name = field
		.file_name()
		.ok_or(AppError::bad_request("Invalid file name"))?
		.to_string();
	let form_bytes = field
		.bytes()
		.await
		.map_err(|_| AppError::bad_request("Invalid image file"))?;

	let uploaded =
		external::image::upload_image(&state.secrets.pictrs_url, &form_bytes, &file_name)
			.await
			.map_err(|_| AppError::bad_request("Image upload failed"))?;
	if uploaded.files.len() != 1 {
		return Err(AppError::internal("Image upload failed"));
	}
	let uploaded = uploaded.files.first().unwrap();
	let uploaded_id = Uuid::parse_str(
		uploaded
			.file
			.split_once(".")
			.ok_or(AppError::internal("Image uploaded failed"))?
			.0,
	)
	.map_err(|_| AppError::internal("Image upload failed"))?;
	let delete_token = Uuid::parse_str(&uploaded.delete_token)
		.map_err(|_| AppError::internal("Image upload failed"))?;

	let inserted = models::image::Image::create(&state.pool, &uploaded_id, &delete_token).await?;

	Ok(Json(UploadImageResponse { id: inserted.id }))
}
