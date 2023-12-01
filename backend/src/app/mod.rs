mod auth;
mod db;
mod errors;
mod routes;
use auth::mw_require_auth;
use axum::http::StatusCode;
use axum::middleware;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use routes::users::router as users_router;
use sqlx::postgres::PgPool;
use tower_cookies::{CookieManagerLayer, Cookies};
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
pub struct AppState {
    pool: PgPool,
}

pub fn api_router() -> Router<AppState> {
    Router::new()
        .route("/cookies", get(read_cookies))
        .layer(middleware::from_fn(mw_require_auth))
        .nest("/users", users_router())
}

pub async fn app() -> Router {
    let pool = db::db_pool().await.unwrap();
    let state = AppState { pool };
    Router::new()
        .route("/", get(probe))
        .nest("/api", api_router())
        .with_state(state)
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
