use crate::app::auth;
use crate::app::auth::{decode_jwt, generate_jwt};
use crate::app::errors::{Error, Result};
use crate::app::routes::AUTH_TOKEN;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::json;
use tower_cookies::{Cookie, Cookies};

pub fn router() -> Router {
    Router::new()
        .route("/api/users/login", post(login))
        .route("/api/users/jwt", get(jwt))
}

#[derive(Deserialize, Debug)]
struct LoginPayload {
    username: String,
    password: String,
}

async fn login(cookies: Cookies, Json(payload): Json<LoginPayload>) -> Result<impl IntoResponse> {
    let jwt = auth::login(&payload.username, &payload.password).await?;

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

#[derive(Deserialize, Debug)]
struct QueryParams {
    username: String,
}

async fn jwt(Query(params): Query<QueryParams>) -> Result<impl IntoResponse> {
    let username = params.username;
    let token = generate_jwt(&username).unwrap();
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
