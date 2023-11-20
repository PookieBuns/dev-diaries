use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("auth error")]
    AuthError,
    #[error("jwt error")]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error("unknown error")]
    Unknown,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}
