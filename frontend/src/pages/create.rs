use crate::components::form_items::{JobApplicationFormItem, LeetcodeFormItem};
use thiserror::Error;
use crate::components::DynamicForm;
use crate::components::FormItem;
use leptos::*;
use logging;
use serde_json::{json, Value};
use crate::utils::base_url;
use leptos::error::Result;

#[derive(Error, Debug)]
pub enum CreateError {
    #[error("create diary failed")]
    CreateDiaryFailed,
}

struct Form {
    leetcode: RwSignal<Vec<LeetcodeFormItem>>,
    job_application: RwSignal<Vec<JobApplicationFormItem>>,
}

async fn create_diary(json_data: Value) -> Result<()> {
    let client = reqwest::Client::new();
    let res = client.post(base_url() + "/api/diary/create")
        .json(&json_data)
        .send()
        .await?;
    let response_code = res.status();
    if !response_code.is_success() {
        res.json().await?;
        return Err(CreateError::CreateDiaryFailed.into());
    }
    logging::log!("response_code: {}", response_code);
    Ok(())
}

#[component]
pub fn Create() -> impl IntoView {
    let form_data = Form {
        leetcode: RwSignal::new(vec![]),
        job_application: RwSignal::new(vec![]),
    };
    let handle_submit = move |_| {
        let mut json_data = json!({});
        let leetcode_data = form_data.leetcode.get().iter().map(|item| item.data()).collect::<Vec<Value>>();
        let job_application_data = form_data.job_application.get().iter().map(|item| item.data()).collect::<Vec<Value>>();
        json_data["leet_code_problems"] = json!(leetcode_data);
        json_data["job_applications"] = json!(job_application_data);
        json_data["user_id"] = json!(1); // TODO: [1] get user id from session
        logging::log!("{}", json_data.to_string());
        spawn_local(
            async move {
                if create_diary(json_data).await.is_ok() {
                    logging::log!("create diary success");
                } else {
                    logging::log!("create diary failed");
                }
            }
        )

    };
    view! {
        <div>
            <h1>"Create"</h1>
            <h2>"Leetcode"</h2>
            <DynamicForm<LeetcodeFormItem > form_items=form_data.leetcode/>
            <h2>"Job Application"</h2>
            <DynamicForm<JobApplicationFormItem> form_items=form_data.job_application/>
        </div>
        <button on:click=handle_submit>"Log Data"</button>
    }
}



