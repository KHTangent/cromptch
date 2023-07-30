mod api;
mod error;
mod models;

use std::sync::Arc;

use axum::{routing::get, Router};
use sqlx::PgPool;
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
		.route("/", get(hello_world))
		.merge(api::user::user_router(app_state.clone()))
		.merge(api::recipe::recipe_router(app_state.clone()));
	info!("Routes created!");

	info!("Server started!");
	Ok(router.into())
}

async fn hello_world() -> &'static str {
	"Hello, world!"
}
