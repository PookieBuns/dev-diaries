use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("auth error")]
    Auth,
    #[error("unknown error")]
    Unknown,

    // Errors from external crates
    #[error("jwt error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("ring error: {0}")]
    Ring(#[from] ring::error::Unspecified),
    #[error("mail error: {0}")]
    Mail(#[from] lettre::error::Error),
    #[error("address error: {0}")]
    Address(#[from] lettre::address::AddressError),
    #[error("smtp error: {0}")]
    Smtp(#[from] lettre::transport::smtp::Error),
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let response_body = Json(json!({
            "error": self.to_string(),
        }));
        (StatusCode::INTERNAL_SERVER_ERROR, response_body).into_response()
    }
}
