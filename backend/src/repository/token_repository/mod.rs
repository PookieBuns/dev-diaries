mod mem_token_repository;

use crate::Result;
use axum::async_trait;
pub use mem_token_repository::MemTokenRepo;

#[async_trait]
pub trait TokenRepo {
    async fn create(&self, token: &str) -> Result<()>;
    async fn find_by_token(&self, token: &str) -> Result<()>;
}
