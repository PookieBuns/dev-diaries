use crate::models::Submission;
use crate::repository::leet_code_repository::LeetCodeRepo;
use crate::Result;

#[derive(Clone)]
pub struct LeetCodeService<T>
where
    T: LeetCodeRepo,
{
    leet_code_repository: T,
}

impl<T> LeetCodeService<T>
where
    T: LeetCodeRepo,
{
    pub fn new(leet_code_repository: T) -> Self {
        Self {
            leet_code_repository,
        }
    }

    pub async fn get_submissions(
        &self,
        session_token: &str,
        offset: i32,
        limit: i32,
    ) -> Result<Vec<Submission>> {
        self.leet_code_repository
            .get_submissions(session_token, offset, limit)
            .await
    }
}
