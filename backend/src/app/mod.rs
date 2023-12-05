mod auth;
mod db;
mod errors;
mod model;
mod password_recovery;
mod repository;
mod routes;
mod service;

use crate::app::repository::user_repository::PgUserRepo;
use crate::app::service::UserService;
use auth::mw_require_auth;
use axum::http::StatusCode;
use axum::middleware;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
pub use errors::Result;
use routes::users::router as users_router;
use tower_cookies::{CookieManagerLayer, Cookies};
use tower_http::cors::{Any, CorsLayer};

// Use this to configure the type of the user repository.
type UserRepoImpl = PgUserRepo;

#[derive(Clone)]
struct AppState {
    user_service: UserService<UserRepoImpl>,
}

fn api_router() -> Router<AppState> {
    Router::new()
        .route("/cookies", get(read_cookies))
        .layer(middleware::from_fn(mw_require_auth))
        .nest("/users", users_router())
}

pub async fn app() -> Router {
    let pool = db::db_pool().await.unwrap();
    let user_repo = UserRepoImpl::new(pool.clone());
    let user_service = UserService::new(user_repo);
    let app_state = AppState { user_service };
    Router::new()
        .route("/", get(probe))
        .nest("/api", api_router())
        .with_state(app_state)
        .layer(CookieManagerLayer::new())
        .layer(CorsLayer::new().allow_origin(Any).allow_headers(Any))
        .fallback(not_found)
}

async fn probe() -> &'static str {
    "OK"
}

async fn read_cookies(cookies: Cookies) -> String {
    let mut cookies_str = String::new();
    for cookie in cookies.list() {
        cookies_str.push_str(&format!("{}: {}\n", cookie.name(), cookie.value()));
    }
    cookies_str
}

async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not Found")
}
