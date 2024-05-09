use crate::utils::base_url;
use leptos::error::Result;
use leptos::*;
use serde_json::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DiariesError {
    #[error("Failed to get diaries")]
    GetDiariesFailed,
    #[error("Failed to create diary")]
    CreateDiaryFailed,
}

pub async fn get_diaries() -> Result<Value> {
    let client = reqwest::Client::new();
    let res = client.get(base_url() + "/api/diary/get").send().await?;
    let response_code = res.status();
    let res_json = res.json().await?;
    if !response_code.is_success() {
        return Err(DiariesError::GetDiariesFailed.into());
    }
    Ok(res_json)
}

pub async fn create_diary(json_data: Value) -> Result<Value> {
    let client = reqwest::Client::new();
    let res = client
        .post(base_url() + "/api/diary/create")
        .json(&json_data)
        .send()
        .await?;
    let response_code = res.status();
    let res_json = res.json().await?;
    if !response_code.is_success() {
        return Err(DiariesError::CreateDiaryFailed.into());
    }
    logging::log!("response_code: {}", response_code);
    Ok(res_json)
}
