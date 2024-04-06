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
pub fn Diaries() -> impl IntoView {
    let (diaries, set_diaries) = create_signal(Value::Null);
    spawn_local(async move {
        let diaries = get_diaries().await.unwrap_or_default();
        set_diaries.set(diaries);
    });
    view! {
        <div>
            <h1>Diaries</h1>
            <p>Welcome to the diaries page!</p>
            <p>Here are your diaries:</p>
            {move || {
                if let Value::Array(diaries) = diaries.get() {
                    view! {
                        <ul>
                            {diaries
                                .iter()
                                .map(|diary| view! { <li>{diary.to_string()}</li> })
                                .collect_view()}

                        </ul>
                    }
                        .into_view()
                } else {
                    view! { <p>"No diaries found"</p> }.into_view()
                }
            }}

        </div>
    }
}
