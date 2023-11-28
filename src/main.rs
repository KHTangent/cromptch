mod api;
mod error;
mod external;
mod models;

use std::env;
use std::str::FromStr;
use std::{net::SocketAddr, sync::Arc};

use axum::{
	http::{self, HeaderValue, Method},
	routing::get,
	Router,
};
use sqlx::PgPool;
use tower_http::cors::CorsLayer;
use tracing::info;

pub struct Config {
	pub hcaptcha_site_key: String,
	pub hcaptcha_secret: String,
	pub pictrs_url: String,
}

pub struct AppState {
	pub pool: PgPool,
	pub secrets: Config,
}

#[tokio::main]
async fn main() {
	env_logger::init();

	info!("Starting server...");
	dotenv::dotenv().ok();

	let secrets = Config {
		hcaptcha_site_key: env::var("HCAPTCHA_SITE_KEY").unwrap_or("".to_string()),
		hcaptcha_secret: env::var("HCAPTCHA_SECRET").unwrap_or("".to_string()),
		pictrs_url: env::var("PICTRS_URL").unwrap_or("".to_string()),
	};
	if secrets.hcaptcha_site_key.is_empty() {
		info!("HCAPTCHA_SITE_KEY not set, captcha will not be used");
	}
	if secrets.pictrs_url.is_empty() {
		info!("PICTRS_URL not set, image features will be disabled");
	}

	info!("Connecting to database...");
	let pool = PgPool::connect(&env::var("POSTGRES_URL").expect("DATABASE_URL not set"))
		.await
		.expect("Failed to connect to database");
	info!("Connected to database!");

	info!("Running database migrations...");
	sqlx::migrate!()
		.run(&pool)
		.await
		.expect("Migrations failed to run");
	info!("Database migrations complete!");

	let app_state = Arc::new(AppState { pool, secrets });

	info!("Creating routes...");
	let router = Router::new()
		.route("/api", get(index))
		.merge(api::user::user_router(app_state.clone()))
		.merge(api::recipe::recipe_router(app_state.clone()))
		.merge(api::image::image_router(app_state.clone()))
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

	let port = env::var("PORT").unwrap_or("3001".to_string());
	info!("Starting server on port {}", port);
	let addr = SocketAddr::from((
		[127, 0, 0, 1],
		u16::from_str(&port).expect("Invalid port number"),
	));
	axum::Server::bind(&addr)
		.serve(router.into_make_service())
		.await
		.expect("Failed to start server");
}

async fn index() -> &'static str {
	"Cromptch API. Please use the frontend instead."
}
