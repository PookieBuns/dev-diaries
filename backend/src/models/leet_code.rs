use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Submission {
    pub id: String,
    pub is_pending: String,
    pub lang: String,
    pub memory: Option<String>,
    pub runtime: Option<String>,
    pub status_display: String,
    pub time: String,
    pub timestamp: String,
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
