use crate::app::errors::{Error, Result};
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use tower_cookies::Cookies;

use crate::app::routes::AUTH_TOKEN;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    sub: String,
    name: String,
    iat: usize,
    exp: usize,
}

const SECRET_KEY: &str = "MY_SECRET_KEY";

pub async fn mw_require_auth<B>(
    cookies: Cookies,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
    println!("auth_token: {:?}", auth_token);
    auth_token.ok_or(Error::Unknown)?;
    Ok(next.run(req).await)
}

pub async fn login(username: &str, password: &str) -> Result<String> {
    if password != "pass" {
        return Err(Error::AuthError);
    }
    let jwt = generate_jwt(&username)?;
    Ok(jwt)
}

pub fn generate_jwt(user_name: &str) -> Result<String> {
    let now = Utc::now().timestamp();
    let one_hour_later = now + 3600;
    let claims = Claims {
        sub: user_name.to_string(),
        name: user_name.to_string(),
        iat: now as usize,
        exp: one_hour_later as usize,
    };
    let key = EncodingKey::from_secret(SECRET_KEY.as_ref());
    let header = Header::default();
    let token = encode(&header, &claims, &key)?;
    Ok(token)
}

pub fn decode_jwt(jwt: &str) -> Result<Claims> {
    let key = DecodingKey::from_secret(SECRET_KEY.as_ref());
    let validation = Validation::default();
    let token = decode::<Claims>(jwt, &key, &validation)?;
    Ok(token.claims)
}
