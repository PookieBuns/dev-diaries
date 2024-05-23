mod pg_diary_repository;

use crate::models::diary::UserDiary;
use crate::models::Submission;
use crate::Result;
use axum::async_trait;

pub use pg_diary_repository::PgDiaryRepo;

#[async_trait]
pub trait DiaryRepo {
    async fn create(&self, diary: &UserDiary) -> Result<i32>;
    async fn get(&self, user_id: i32) -> Result<Vec<UserDiary>>;
    async fn get_latest_leet_code_submission(&self, user_id: i32) -> Result<i64>;
    async fn insert_leet_code_submissions(
        &self,
        user_id: i32,
        submissions: &[Submission],
    ) -> Result<()>;
}
