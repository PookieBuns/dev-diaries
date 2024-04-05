use crate::model::Diary;
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

    pub async fn create_diary(&self, mut diary: Diary) -> Result<()> {
        if diary.diary_date.is_none() {
            diary.diary_date = Some(chrono::Utc::now().date_naive());
        }
        self.diary_repository.create(&diary).await?;
        Ok(())
    }
}
