use bytes::Bytes;
use tracing::{info, warn};
use uuid::Uuid;

use crate::error::{AppError, AppResult};

pub async fn get_image_bytes(pictrs_url: &str, id: &Uuid) -> AppResult<axum::body::Bytes> {
	let full_picture_url = format!("{}/image/original/{}.webp", pictrs_url, id);
	info!("Fetching image from {}", full_picture_url);
	let response = reqwest::get(&full_picture_url)
		.await
		.map_err(|_| AppError::not_found("Image not found"))?;
	let bytes = response
		.bytes()
		.await
		.map_err(|_| AppError::internal("Error fetching image"))?;
	Ok(bytes)
}

pub async fn get_thumbnail_bytes(pictrs_url: &str, id: &Uuid) -> AppResult<axum::body::Bytes> {
	let thumbnail_url = format!(
		"{}/image/process.webp?thumbnail=200&src={}.webp",
		pictrs_url, id
	);
	info!("Fetching image from {}", thumbnail_url);
	let response = reqwest::get(&thumbnail_url)
		.await
		.map_err(|_| AppError::not_found("Image not found"))?;
	let bytes = response
		.bytes()
		.await
		.map_err(|_| AppError::internal("Error fetching image"))?;
	Ok(bytes)
}

#[derive(serde::Deserialize)]
pub struct ImageUploadDetails {
	pub content_type: String,
	pub created_at: String,
	pub format: String,
	pub frames: Option<u32>,
	pub height: u32,
	pub width: u32,
}

#[derive(serde::Deserialize)]
pub struct ImageUploadFile {
	pub delete_token: String,
	pub details: ImageUploadDetails,
	pub file: String,
}

#[derive(serde::Deserialize)]
pub struct ImageUploadResponse {
	pub files: Vec<ImageUploadFile>,
	pub msg: String,
}

pub async fn upload_image(
	pictrs_url: &str,
	image_bytes: &Bytes,
	filename: &str,
) -> AppResult<ImageUploadResponse> {
	let file_part =
		reqwest::multipart::Part::bytes(image_bytes.to_vec()).file_name(filename.to_string());
	let form = reqwest::multipart::Form::new().part("images[]", file_part);
	info!("Uploading image {}", filename);
	let response = reqwest::Client::new()
		.post(&format!("{}/image", pictrs_url))
		.multipart(form)
		.send()
		.await
		.map_err(|_| AppError::bad_request("Invalid image"))?;
	if !response.status().is_success() {
		warn!(
			"Image upload received status {}: {}",
			response.status(),
			response.text().await.unwrap_or("(unknown)".to_string())
		);
		return Err(AppError::bad_request("Invalid image"));
	}
	let response = response
		.json::<ImageUploadResponse>()
		.await
		.map_err(|_| AppError::internal("Failed to parse image response"))?;
	if response.msg != "ok" {
		warn!(
			"Image upload received incorrect status message: {}",
			&response.msg
		);
		return Err(AppError::bad_request("Error uploading image"));
	}
	info!("Image uploaded successfully");
	Ok(response)
}
