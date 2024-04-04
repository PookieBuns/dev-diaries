use crate::components::form_items::{JobApplicationFormItem, LeetcodeFormItem};
use crate::components::DynamicForm;
use crate::components::FormItem;
use leptos::*;
use logging;
use serde_json::{json, Value};

struct Form {
    leetcode: RwSignal<Vec<LeetcodeFormItem>>,
    job_application: RwSignal<Vec<JobApplicationFormItem>>,
}

#[component]
pub fn Create() -> impl IntoView {
    let form_data = Form {
        leetcode: RwSignal::new(vec![]),
        job_application: RwSignal::new(vec![]),
    };
    view! {
        <div>
            <h1>"Create"</h1>
            <h2>"Leetcode"</h2>
            <DynamicForm<LeetcodeFormItem > form_items=form_data.leetcode/>
            <h2>"Job Application"</h2>
            <DynamicForm<JobApplicationFormItem> form_items=form_data.job_application/>
        </div>
        <button on:click=move |_| {
            let mut json_data = json!({});
            let leetcode_data = form_data.leetcode.get().iter().map(|item| item.data()).collect::<Vec<Value>>();
            let job_application_data = form_data.job_application.get().iter().map(|item| item.data()).collect::<Vec<Value>>();
            json_data["leetcode"] = json!(leetcode_data);
            json_data["job_application"] = json!(job_application_data);
            logging::log!("{}", json_data.to_string());
        }>"Log Data"</button>
    }
}

