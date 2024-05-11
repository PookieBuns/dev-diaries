use futures::future::try_join_all;
use futures::StreamExt;
use futures::TryStreamExt;

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
        // split the range into 20s
        let res = (offset..offset + limit).step_by(20).map(|x| {
            self.leet_code_repository
                .get_submissions(session_token, x, limit.min(x + 20))
        });
        Ok(futures::stream::iter(res)
            .buffered(5)
            .try_collect::<Vec<_>>()
            .await?
            .into_iter()
            .map(|x| x.submissions)
            .flatten()
            .collect())
        // Ok(try_join_all(res).await?.into_iter().flatten().collect())
    }

    pub async fn get_all_submissions(&self, session_token: &str) -> Result<Vec<Submission>> {
        let res = self.get_submissions(session_token, 0, 20).await?;
        Ok(res)
    }
}
