use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Submission {
    #[serde(deserialize_with = "deserialize_number_from_string", alias = "id")]
    pub leet_code_submission_id: i64,
    pub is_pending: String,
    pub lang: String,
    pub memory: Option<String>,
    pub runtime: Option<String>,
    pub status_display: String,
    pub time: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub timestamp: i64,
    pub title: String,
    pub title_slug: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SubmissionList {
    pub has_next: bool,
    pub submissions: Vec<Submission>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Data {
    pub submission_list: SubmissionList,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct LeetCodeSubmissionListResponse {
    pub data: Data,
}
