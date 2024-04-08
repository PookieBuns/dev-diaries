use crate::utils::base_url;
use leptos::error::Result;
use leptos::*;
use reqwest;
use serde_json::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DiariesError {
    #[error("Failed to get diaries")]
    GetDiariesFailed,
}

async fn get_diaries() -> Result<Value> {
    let client = reqwest::Client::new();
    let res = client.get(base_url() + "/api/diary/get").send().await?;
    let response_code = res.status();
    let res_json = res.json().await?;
    if !response_code.is_success() {
        return Err(DiariesError::GetDiariesFailed.into());
    }
    Ok(res_json)
}

#[component]
pub fn DiaryCard(diary_data: Value) -> impl IntoView {
    view! {
        <div class="card mb-3">
            <div class="card-header">
                <h5>{diary_data["diary_date"].as_str().unwrap().to_owned()}</h5>
            // <p>{diary_data.to_string()}</p>
            </div>
            <div class="card-header">LeetCode Problems</div>
            <ul class="list-group list-group-flush">
                {diary_data["leet_code_problems"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|problem| {
                        view! { <li class="list-group-item">{problem.to_string()}</li> }
                    })
                    .collect_view()}
            </ul>
            <div class="card-header">Job Applications</div>
            <ul class="list-group list-group-flush">
                {diary_data["job_applications"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|problem| {
                        view! { <li class="list-group-item">{problem.to_string()}</li> }
                    })
                    .collect_view()}
            </ul>
        </div>
    }
}

#[component]
pub fn Diaries() -> impl IntoView {
    let (diaries, set_diaries) = create_signal(Value::Null);
    spawn_local(async move {
        let diaries = get_diaries().await.unwrap_or_default();
        set_diaries.set(diaries);
    });
    view! {
        <h1>Diaries</h1>
        <p>Welcome to the diaries page!</p>
        <p>Here are your diaries:</p>
        {move || {
            if let Value::Array(diaries) = diaries.get() {
                view! {
                    <ul class="px-0">
                        {diaries
                            .into_iter()
                            .map(|diary| {
                                view! { <DiaryCard diary_data=diary/> }
                            })
                            .collect_view()}

                    </ul>
                }
                    .into_view()
            } else {
                view! { <p>"No diaries found"</p> }.into_view()
            }
        }}
    }
}










