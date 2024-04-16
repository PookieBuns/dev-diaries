use crate::api::diary::{create_diary, get_diaries};
use crate::components::form_items::{JobApplicationFormItem, LeetcodeFormItem};
use crate::components::DynamicForm;
use crate::components::MarkdownInput;
use leptos::ev::SubmitEvent;
use leptos::*;
use leptos_router::*;
use serde_json::{json, Value};

struct Form {
    id: RwSignal<Option<u64>>,
    leetcode: RwSignal<Vec<LeetcodeFormItem>>,
    job_application: RwSignal<Vec<JobApplicationFormItem>>,
    notes: RwSignal<String>,
}

#[component]
pub fn Create() -> impl IntoView {
    let form_data = Form {
        id: RwSignal::new(None),
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
            .map(|item| serde_json::to_value(item).unwrap())
            .collect::<Vec<Value>>();
        let job_application_data = form_data
            .job_application
            .get()
            .iter()
            .map(|item| serde_json::to_value(item).unwrap())
            .collect::<Vec<Value>>();
        json_data["diary_date"] = json!(chrono::Local::now().date_naive().to_string());
        json_data["leet_code_problems"] = json!(leetcode_data);
        json_data["job_applications"] = json!(job_application_data);
        json_data["diary_notes"] = json!(form_data.notes.get());
        logging::log!("id: {:?}", form_data.id);
        json_data["diary_id"] = json!(form_data.id);
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
    spawn_local(async move {
        let diaries = get_diaries().await;
        if let Ok(diaries) = diaries {
            logging::log!("{}", diaries.to_string());
            let diaries = diaries.as_array().unwrap();
            let today = chrono::Local::now().date_naive().to_string();
            let today_diary = diaries
                .iter()
                .rev()
                .find(|diary| diary["diary_date"].as_str().unwrap() == today);
            if let Some(today_diary) = today_diary {
                let leet_code_problems: Vec<LeetcodeFormItem> =
                    serde_json::from_value(today_diary["leet_code_problems"].clone()).unwrap();
                form_data.leetcode.set(leet_code_problems);
                let job_applications: Vec<JobApplicationFormItem> =
                    serde_json::from_value(today_diary["job_applications"].clone()).unwrap();
                form_data.job_application.set(job_applications);
                let notes = today_diary["diary_notes"].as_str().unwrap();
                form_data.notes.set(notes.to_string());
                form_data
                    .id
                    .set(Some(today_diary["diary_id"].as_u64().unwrap()));
            }
        }
    });
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
