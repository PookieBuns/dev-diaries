use crate::mw_auth::mw_require_auth;
use crate::routes::users::router as users_router;
use axum::http::StatusCode;
use axum::middleware;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use tower_cookies::{CookieManagerLayer, Cookies};

pub fn api_router() -> Router {
    Router::new()
        .route("/", get(probe))
        .route("/cookies", get(read_cookies))
        .layer(middleware::from_fn(mw_require_auth))
        .merge(users_router())
        .layer(CookieManagerLayer::new())
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
