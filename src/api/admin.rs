use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{error::AppResult, models::admin::Admin, AppState};

pub fn admin_router(state: Arc<AppState>) -> Router {
	Router::new()
		.route("/api/admin/self", get(get_admin_self))
		.with_state(state)
}

async fn get_admin_self(_: Admin) -> AppResult<&'static str> {
	Ok("Congratulations, you are an administrator :)")
}
