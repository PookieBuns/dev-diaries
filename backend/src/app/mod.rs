mod auth;
mod errors;
mod routes;
use auth::mw_require_auth;
use axum::http::StatusCode;
use axum::middleware;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use routes::users::router as users_router;
use tower_cookies::{CookieManagerLayer, Cookies};
use tower_http::cors::{Any, CorsLayer};

pub fn api_router() -> Router {
    Router::new()
        .route("/cookies", get(read_cookies))
        .layer(middleware::from_fn(mw_require_auth))
        .route("/", get(probe))
        .merge(users_router())
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
