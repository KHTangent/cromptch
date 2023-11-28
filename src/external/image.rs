use uuid::Uuid;

use crate::error::{AppError, AppResult};

pub async fn get_image_bytes(pictrs_url: &str, id: &Uuid) -> AppResult<axum::body::Bytes> {
	let full_picture_url = format!("{}/image/original/{}.webp", pictrs_url, id);
	println!("Fetching image from {}", full_picture_url);
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
	println!("Fetching image from {}", thumbnail_url);
	let response = reqwest::get(&thumbnail_url)
		.await
		.map_err(|_| AppError::not_found("Image not found"))?;
	let bytes = response
		.bytes()
		.await
		.map_err(|_| AppError::internal("Error fetching image"))?;
	Ok(bytes)
}
