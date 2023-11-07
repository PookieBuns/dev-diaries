use crate::error::{Error, Result};
use crate::routes::AUTH_TOKEN;
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

async fn login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    if payload.username != "user" || payload.password != "pass" {
        return Err(Error::LoginFail);
    }

    let mut cookie = Cookie::new(AUTH_TOKEN, "test-token");
    cookie.set_path("/");
    cookies.add(cookie);

    let body = Json(json!({
        "result": {
            "success": true,
        }
    }));
    Ok(body)
}
