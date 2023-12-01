use crate::app::errors::{Error, Result};
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::num::NonZeroU32;
use tower_cookies::Cookies;

use crate::app::routes::AUTH_TOKEN;

const EXPIRATION_TIME: i64 = 3600;

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
    auth_token.ok_or(Error::Auth)?;
    Ok(next.run(req).await)
}

#[derive(sqlx::FromRow, Debug)]
struct PasswordHash {
    salt: [u8; digest::SHA256_OUTPUT_LEN],
    #[sqlx(rename = "password_hash")]
    hash: [u8; digest::SHA256_OUTPUT_LEN],
}

fn hash_password(password: &str) -> Result<PasswordHash> {
    const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
    let rng = rand::SystemRandom::new();
    let mut salt = [0u8; CREDENTIAL_LEN];
    rng.fill(&mut salt)?;

    let mut hash = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        NonZeroU32::new(100_000).unwrap(),
        &salt,
        password.as_bytes(),
        &mut hash,
    );
    Ok(PasswordHash { salt, hash })
}

pub fn verify_password(password: &str, salt: &[u8], hash: &[u8]) -> bool {
    let result = pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        NonZeroU32::new(100_000).unwrap(),
        salt,
        password.as_bytes(),
        hash,
    );
    result.is_ok()
}

#[derive(sqlx::FromRow, Debug)]
struct User {
    user_id: i32,
    // user_name: String,
    #[sqlx(flatten)]
    password: PasswordHash,
}

pub async fn login(username: &str, password: &str, pool: &PgPool) -> Result<String> {
    let user: User = sqlx::query_as("SELECT * FROM \"user\" WHERE user_name = $1")
        .bind(username)
        .fetch_one(pool)
        .await?;
    println!("user: {:?}", user);
    if !verify_password(password, &user.password.salt, &user.password.hash) {
        return Err(Error::Auth);
    }
    let jwt = generate_jwt(username, user.user_id)?;
    Ok(jwt)
}

pub async fn register(username: &str, password: &str, pool: &PgPool) -> Result<()> {
    let hash = hash_password(password)?;
    sqlx::query(
        "INSERT INTO \"user\" (user_name, password_hash, salt)
        VALUES ($1, $2, $3)",
    )
    .bind(username)
    .bind(hash.hash)
    .bind(hash.salt)
    .execute(pool)
    .await?;
    Ok(())
}

pub fn generate_jwt(user_name: &str, user_id: i32) -> Result<String> {
    let now = Utc::now().timestamp();
    let exp = now + EXPIRATION_TIME;
    let claims = Claims {
        sub: user_id.to_string(),
        name: user_name.to_string(),
        iat: now as usize,
        exp: exp as usize,
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
