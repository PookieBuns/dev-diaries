use crate::models::diary::{Diary, UserDiary};
use crate::repository::diary_repository::DiaryRepo;
use crate::Result;

#[derive(Clone)]
pub struct DiaryService<T>
where
    T: DiaryRepo,
{
    diary_repository: T,
}

impl<T> DiaryService<T>
where
    T: DiaryRepo,
{
    pub fn new(diary_repository: T) -> Self {
        Self { diary_repository }
    }

    pub async fn create_diary(&self, user_id: i32, diary: Diary) -> Result<i32> {
        let user_diary = UserDiary { user_id, diary };
        Ok(self.diary_repository.create(&user_diary).await?)
    }

    pub async fn get_diaries(&self, user_id: i32) -> Result<Vec<UserDiary>> {
        self.diary_repository.get(user_id).await
    }
}
