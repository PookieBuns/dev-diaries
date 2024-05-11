// use futures::future::try_join_all;
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
            .flat_map(|x| x.submissions)
            .collect())
        // Ok(try_join_all(res).await?.into_iter().flatten().collect())
    }

    pub async fn get_all_submissions(&self, session_token: &str) -> Result<Vec<Submission>> {
        let mut offset = 0;
        let mut limit = offset + 20;
        let mut submission_list = self
            .leet_code_repository
            .get_submissions(session_token, offset, limit)
            .await?;
        let mut submissions = submission_list.submissions;
        while submission_list.has_next {
            offset = limit;
            limit = offset + 20;
            submission_list = self
                .leet_code_repository
                .get_submissions(session_token, offset, limit)
                .await?;
            submissions.extend(submission_list.submissions);
        }
        Ok(submissions)
    }
}
