use crate::routes::AUTH_TOKEN;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

pub fn router() -> Router {
    Router::new().route("/api/users/login", post(login))
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

async fn login(cookies: Cookies, payload: Json<LoginPayload>) -> impl IntoResponse {
    println!("login: {:?}", payload);
    if payload.username != "user" || payload.password != "pass" {
        return (StatusCode::UNAUTHORIZED, "Login failed").into_response();
    }

    let mut cookie = Cookie::new(AUTH_TOKEN, "test-token");
    cookie.set_path("/");
    cookies.add(cookie);

    let body = Json(json!({
        "result": {
            "success": true,
        }
    }));
    (StatusCode::OK, body).into_response()
}
