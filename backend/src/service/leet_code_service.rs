use crate::Result;
use reqwest::Client;
use serde_json::{json, Value};

const BASE_URL: &str = "https://leetcode.com/graphql";

#[derive(Clone)]
pub struct LeetCodeService {
    client: Client,
}

impl LeetCodeService {
    pub fn new() -> Self {
        Self {
            client: Client::builder().cookie_store(true).build().unwrap(),
        }
    }

    pub async fn get_submissions(&self, session_token: &str) -> Result<Value> {
        let graphql_query = json!({
            "query": r#"{ submissionList(offset: 0, limit: 20, questionSlug: "") { hasNext submissions { id lang time timestamp statusDisplay runtime url isPending title memory titleSlug } } }"#,
        });
        let cookie = format!("LEETCODE_SESSION={session_token}");
        let res = self
            .client
            .post(BASE_URL)
            .header("Cookie", cookie)
            .json(&graphql_query)
            .send()
            .await?;
        Ok(res.json().await?)
    }
}
