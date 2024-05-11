mod http_leet_code_repository;

use crate::models::Submission;
use crate::Result;
use axum::async_trait;
pub use http_leet_code_repository::HttpLeetCodeRepo;

#[async_trait]
pub trait LeetCodeRepo {
    async fn get_submissions(
        &self,
        session_token: &str,
        offset: i32,
        limit: i32,
    ) -> Result<Vec<Submission>>;
}
