use leptos::*;
use leptos::error::Result;
use thiserror::Error;
use web_sys::Storage;

#[derive(Debug, Clone, Error)]
pub enum AuthError {
    #[error("storage error")]
    Storage,
}

pub fn get_local_storage() -> Result<Storage> {
    let window = window();
    let local_storage = window
        .local_storage()
        .map_err(|_| AuthError::Storage)?
        .ok_or(AuthError::Storage)?;
    Ok(local_storage)
}

