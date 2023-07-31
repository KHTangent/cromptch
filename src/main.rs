mod api;
mod error;
mod models;

use std::sync::Arc;

use axum::{
	http::{self, HeaderValue, Method},
	routing::get,
	Router,
};
use shuttle_secrets::SecretStore;
use sqlx::PgPool;
use tower_http::cors::CorsLayer;
use tracing::info;

pub struct AppState {
	pub pool: PgPool,
	pub secret_store: SecretStore,
}

#[shuttle_runtime::main]
async fn axum(
	#[shuttle_shared_db::Postgres] pool: PgPool,
	#[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_axum::ShuttleAxum {
	info!("Starting server...");

	if let Some(_) = secret_store.get("HCAPTCHA_SITE_KEY") {
		if let Some(_) = secret_store.get("HCAPTCHA_SECRET") {
			info!("hCaptcha enabled");
		} else {
			panic!("hCaptcha secret not found");
		}
	} else {
		info!("hCaptcha disabled");
	}

	info!("Running database migrations...");
	sqlx::migrate!()
		.run(&pool)
		.await
		.expect("Migrations failed to run");
	info!("Database migrations complete!");

	let app_state = Arc::new(AppState { pool, secret_store });

	info!("Creating routes...");
	let router = Router::new()
		.route("/", get(index))
		.merge(api::user::user_router(app_state.clone()))
		.merge(api::recipe::recipe_router(app_state.clone()))
		.layer(
			CorsLayer::new()
				.allow_origin([
					"http://localhost:3000".parse::<HeaderValue>().unwrap(),
					"https://cromptch.derg.vip".parse::<HeaderValue>().unwrap(),
				])
				.allow_headers([http::header::AUTHORIZATION, http::header::CONTENT_TYPE])
				.allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]),
		);
	info!("Routes created!");

	info!("Server started!");
	Ok(router.into())
}

async fn index() -> &'static str {
	"Cromptch API. Please use the frontend instead."
}
