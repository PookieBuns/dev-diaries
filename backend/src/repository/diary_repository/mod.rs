mod pg_diary_repository;

use crate::models::diary::Diary;
use crate::Result;
use axum::async_trait;

pub use pg_diary_repository::PgDiaryRepo;

#[async_trait]
pub trait DiaryRepo {
    async fn create(&self, diary: &Diary) -> Result<()>;
    async fn get(&self, user_id: i32) -> Result<Vec<Diary>>;
}
