use crate::models::user::PasswordHash;
use crate::Result;
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;

pub const AUTH_TOKEN: &str = "auth-token";
const EXPIRATION_TIME: i64 = 3600;
const SECRET_KEY: &str = "MY_SECRET_KEY";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claims {
    sub: i32,
    name: String,
    iat: usize,
    exp: usize,
}

impl Claims {
    pub fn user_id(&self) -> i32 {
        self.sub
    }
}

pub fn hash_password(password: &str) -> Result<PasswordHash> {
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

pub fn generate_jwt(user_name: &str, user_id: i32) -> Result<String> {
    let now = Utc::now().timestamp();
    let exp = now + EXPIRATION_TIME;
    let claims = Claims {
        sub: user_id,
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
