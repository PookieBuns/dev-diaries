use crate::app::auth;
use crate::app::auth::{decode_jwt, generate_jwt};
use crate::app::errors::{Error, Result};
use crate::app::routes::AUTH_TOKEN;
use crate::app::AppState;
use axum::extract::{Query, State};
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
}

#[derive(Deserialize, Debug)]
struct UserPayload {
    username: String,
    password: String,
}

async fn login(State(state): State<AppState>, cookies: Cookies, Json(payload): Json<UserPayload>) -> Result<impl IntoResponse> {
    let db = &state.pool;
    let jwt = auth::login(&payload.username, &payload.password, db).await?;
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

async fn register(State(state): State<AppState>, Json(payload): Json<UserPayload>) -> Result<impl IntoResponse> {
    // cloning the pool doesn't create a new pool
    let db = &state.pool;
    auth::register(&payload.username, &payload.password, db).await?;
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
    let username = params.username;
    let token = generate_jwt(&username, 0).unwrap();
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
