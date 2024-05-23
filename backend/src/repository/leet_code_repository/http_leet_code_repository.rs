use super::LeetCodeRepo;
use crate::{
    models::{LeetCodeSubmissionListResponse, SubmissionList},
    Result,
};
use axum::async_trait;
use reqwest::Client;
use serde_json::json;
use tracing::info;

const BASE_URL: &str = "https://leetcode.com/graphql";

#[derive(Clone)]
pub struct HttpLeetCodeRepo {
    client: Client,
}

impl HttpLeetCodeRepo {
    pub fn new() -> Self {
        Self {
            client: Client::builder().cookie_store(true).build().unwrap(),
        }
    }
}

#[async_trait]
impl LeetCodeRepo for HttpLeetCodeRepo {
    async fn get_submissions(
        &self,
        session_token: &str,
        offset: i32,
        limit: i32,
    ) -> Result<SubmissionList> {
        info!("get_submissions {offset} {limit}");
        let graphql_query = json!({
            "query": format!(r#"{{
                submissionList(offset: {offset}, limit: {limit}, questionSlug: "") 
                {{ hasNext submissions 
                    {{ id lang time timestamp statusDisplay runtime url isPending title memory titleSlug }}
                }}
            }}"#),
        });
        let cookie = format!("LEETCODE_SESSION={session_token}");
        let res: LeetCodeSubmissionListResponse = self
            .client
            .post(BASE_URL)
            .header("Cookie", cookie)
            .json(&graphql_query)
            .send()
            .await?
            .json()
            .await?;
        Ok(res.data.submission_list)
    }
}

impl Default for HttpLeetCodeRepo {
    fn default() -> Self {
        Self::new()
    }
}
