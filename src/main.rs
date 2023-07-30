mod api;
mod error;
mod models;

use std::sync::Arc;

use axum::{
	http::{HeaderValue, Method},
	routing::get,
	Router,
};
use sqlx::PgPool;
use tower_http::cors::CorsLayer;
use tracing::info;

pub struct AppState {
	pub pool: PgPool,
}

#[shuttle_runtime::main]
async fn axum(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
	info!("Starting server...");

	info!("Running database migrations...");
	sqlx::migrate!()
		.run(&pool)
		.await
		.expect("Migrations failed to run");
	info!("Database migrations complete!");

	let app_state = Arc::new(AppState { pool });

	info!("Creating routes...");
	let router = Router::new()
		.layer(
			CorsLayer::new()
				.allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
				.allow_origin("https://cromptch.derg.vip".parse::<HeaderValue>().unwrap())
				.allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]),
		)
		.route("/", get(index))
		.merge(api::user::user_router(app_state.clone()))
		.merge(api::recipe::recipe_router(app_state.clone()));
	info!("Routes created!");

	info!("Server started!");
	Ok(router.into())
}

async fn index() -> &'static str {
	"Cromptch API. Please use the frontend instead."
}
