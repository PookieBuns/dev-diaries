use crate::components::form_items::{JobApplicationFormItem, LeetcodeFormItem};
use crate::components::DynamicForm;
use crate::components::FormItem;
use crate::components::MarkdownInput;
use crate::utils::base_url;
use leptos::error::Result;
use leptos::ev::SubmitEvent;
use leptos::*;
use leptos_router::*;
use logging;
use serde_json::{json, Value};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CreateError {
    #[error("create diary failed")]
    CreateDiaryFailed,
}

struct Form {
    leetcode: RwSignal<Vec<LeetcodeFormItem>>,
    job_application: RwSignal<Vec<JobApplicationFormItem>>,
    notes: RwSignal<String>,
}

async fn create_diary(json_data: Value) -> Result<()> {
    let client = reqwest::Client::new();
    let res = client
        .post(base_url() + "/api/diary/create")
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
        notes: RwSignal::new("".to_string()),
    };
    let handle_submit = move |ev: SubmitEvent| {
        logging::log!("submit");
        ev.prevent_default();
        let navigate = use_navigate();
        let mut json_data = json!({});
        let leetcode_data = form_data
            .leetcode
            .get()
            .iter()
            .map(|item| item.data())
            .collect::<Vec<Value>>();
        let job_application_data = form_data
            .job_application
            .get()
            .iter()
            .map(|item| item.data())
            .collect::<Vec<Value>>();
        json_data["diary_date"] = json!(chrono::Local::now().date_naive().to_string());
        json_data["leet_code_problems"] = json!(leetcode_data);
        json_data["job_applications"] = json!(job_application_data);
        json_data["diary_notes"] = json!(form_data.notes.get());
        logging::log!("{}", json_data.to_string());
        spawn_local(async move {
            if create_diary(json_data).await.is_ok() {
                logging::log!("create diary success");
                navigate("/home", Default::default());
            } else {
                logging::log!("create diary failed");
            }
        })
    };
    view! {
        <form on:submit=handle_submit>
            <h1>Create</h1>
            <h2>Leetcode</h2>
            <DynamicForm<LeetcodeFormItem > form_items=form_data.leetcode/>
            <h2>"Job Application"</h2>
            <DynamicForm<JobApplicationFormItem> form_items=form_data.job_application/>
            <h2>Notes</h2>
            <MarkdownInput value=form_data.notes/>
            <div class="d-grid">
                <button class="btn btn-primary" type="submit">Submit</button>
            </div>
        </form>
    }
}
