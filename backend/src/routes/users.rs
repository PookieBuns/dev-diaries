use crate::app::AppState;
use crate::auth::AUTH_TOKEN;
use crate::errors::{Error, Result};
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::json;
use tower_cookies::{Cookie, Cookies};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/jwt", get(jwt))
        .route("/register", post(register))
        .route("/password-reset", get(get_password_reset_token))
        .route("/password-reset/:token", get(verify_password_reset))
        .route("/test", get(test))
}

#[derive(Deserialize, Debug)]
struct UserPayload {
    username: String,
    password: String,
}

async fn login(
    State(state): State<AppState>,
    cookies: Cookies,
    Json(payload): Json<UserPayload>,
) -> Result<impl IntoResponse> {
    let user_service = &state.user_service;
    let jwt = user_service
        .login(&payload.username, &payload.password)
        .await?;
    let mut cookie = Cookie::new(AUTH_TOKEN, jwt.clone());
    cookie.set_path("/");
    cookies.add(cookie);

    let body = Json(json!({
        "result": {
            "success": true,
            "token": jwt,
        }
    }));
    Ok((StatusCode::OK, body))
}

async fn register(
    State(state): State<AppState>,
    Json(payload): Json<UserPayload>,
) -> Result<impl IntoResponse> {
    // cloning the pool doesn't create a new pool
    let user_service = &state.user_service;
    user_service
        .register(&payload.username, &payload.password)
        .await?;
    let body = Json(json!({
        "result": {
            "success": true,
        }
    }));
    Ok((StatusCode::OK, body))
}

#[derive(Deserialize, Debug)]
struct QueryParams {
    username: String,
}

async fn jwt(Query(params): Query<QueryParams>) -> Result<impl IntoResponse> {
    use crate::auth::{decode_jwt, generate_jwt};
    let username = params.username;
    let token = generate_jwt(&username, 0)?;
    let res = decode_jwt(&token);
    match res {
        Ok(_) => println!("jwt: {:?}", res),
        Err(e) => match e {
            Error::JwtError(jwt_error) => {
                println!("jwt_error: {:?}", jwt_error);
                return Err(Error::JwtError(jwt_error));
            }
            _ => {
                println!("error: {:?}", e);
                return Err(Error::Unknown);
            }
        },
    }
    let body = Json(json!({
        "result": {
            "success": true,
            "token": token,
        }
    }));
    Ok((StatusCode::OK, body))
}

async fn get_password_reset_token(
    State(state): State<AppState>,
    Query(payload): Query<UserPayload>,
) -> Result<impl IntoResponse> {
    let user_service = &state.user_service;
    user_service
        .request_password_reset(&payload.username)
        .await?;
    let body = Json(json!({
        "result": {
            "success": true,
        }
    }));
    Ok((StatusCode::OK, body))
}

async fn verify_password_reset(
    State(state): State<AppState>,
    Path(token): Path<String>,
) -> Result<impl IntoResponse> {
    let user_service = &state.user_service;
    user_service.verify_password_reset(&token).await?;
    let body = Json(json!({
        "result": {
            "success": true,
        }
    }));
    Ok((StatusCode::OK, body))
}

async fn test() -> Result<impl IntoResponse> {
    Ok("success")
}
