use std::collections::HashMap;

use chrono::DateTime;
use chrono::Utc;
// use futures::future::try_join_all;
use futures::StreamExt;
use futures::TryStreamExt;
use tracing::info;

use crate::models::diary::{DifficultyLevel, LeetCodeProblem};
use crate::models::Diary;
use crate::models::Submission;
use crate::repository::diary_repository::DiaryRepo;
use crate::repository::leet_code_repository::LeetCodeRepo;
use crate::Result;

#[derive(Clone)]
pub struct LeetCodeService<T, U>
where
    T: LeetCodeRepo,
    U: DiaryRepo,
{
    leet_code_repository: T,
    diary_repository: U,
}

impl<T, U> LeetCodeService<T, U>
where
    T: LeetCodeRepo,
    U: DiaryRepo,
{
    pub fn new(leet_code_repository: T, diary_repository: U) -> Self {
        Self {
            leet_code_repository,
            diary_repository,
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

    pub async fn sync_submissions(&self, user_id: i32, session_token: &str) -> Result<()> {
        let last_local_submission_timestamp = self
            .diary_repository
            .get_latest_leet_code_submission(user_id)
            .await?;
        let one_year_ago_timestamp = (chrono::Utc::now() - chrono::Duration::days(365)).timestamp();
        let last_sync_timestamp = last_local_submission_timestamp.max(one_year_ago_timestamp);
        info!(last_sync_timestamp = last_sync_timestamp);

        let mut offset = 0;
        let mut limit = offset + 20;
        let mut submission_list = self
            .leet_code_repository
            .get_submissions(session_token, offset, limit)
            .await?;
        let mut submissions = submission_list.submissions;
        let mut submissions_length = submissions.len();
        let mut new_submissions: Vec<Submission> = Vec::new();
        let mut filtered_submissions: Vec<Submission> = submissions
            .into_iter()
            .filter(|x| x.timestamp > last_sync_timestamp)
            .collect();
        let mut filtered_submissions_length = filtered_submissions.len();
        new_submissions.extend(filtered_submissions);

        while submission_list.has_next && filtered_submissions_length == submissions_length {
            offset = limit;
            limit = offset + 20;
            submission_list = self
                .leet_code_repository
                .get_submissions(session_token, offset, limit)
                .await?;
            submissions = submission_list.submissions;
            submissions_length = submissions.len();
            filtered_submissions = submissions
                .into_iter()
                .filter(|x| x.timestamp > last_sync_timestamp)
                .collect();
            filtered_submissions_length = filtered_submissions.len();
            new_submissions.extend(filtered_submissions);
        }
        let earliest_accepted_submissions =
            Self::get_earliest_accepted_submissions(&new_submissions);
        let diary_entries: Vec<Diary> = earliest_accepted_submissions
            .into_iter()
            .map(|(title_slug, (st, at))| Diary {
                diary_id: None,
                diary_date: at.unwrap_or(st).date_naive(),
                diary_notes: "".to_string(),
                leet_code_problems: vec![LeetCodeProblem {
                    leet_code_problem_id: None,
                    diary_id: None,
                    problem_link: format!("https://leetcode.com/problems/{title_slug}"),
                    difficulty: DifficultyLevel::Easy,
                    is_done: at.is_some(),
                }],
                job_applications: vec![],
            })
            .collect();
        println!("{diary_entries:#?}");
        // TODO: insert diary entries into the database
        self.diary_repository
            .insert_leet_code_submissions(user_id, &new_submissions)
            .await?;
        Ok(())
    }

    fn get_earliest_accepted_submissions(
        submissions: &[Submission],
    ) -> HashMap<String, (DateTime<Utc>, Option<DateTime<Utc>>)> {
        submissions
            .iter()
            .fold(HashMap::new(), |mut map, submission| {
                let submission_datetime =
                    DateTime::from_timestamp(submission.timestamp, 0).unwrap();
                let accepted_datetime = (submission.status_display == "Accepted")
                    .then_some(DateTime::from_timestamp(submission.timestamp, 0).unwrap());
                map.entry(submission.title_slug.clone())
                    .and_modify(|(submission_time, accepted_time)| {
                        *submission_time = (*submission_time).min(submission_datetime);
                        *accepted_time = accepted_datetime.map_or(*accepted_time, |x| {
                            accepted_time.map_or(Some(x), |y| Some(x.min(y)))
                        });
                    })
                    .or_insert((submission_datetime, accepted_datetime));
                map
            })
    }
}
