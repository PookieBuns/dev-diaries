use crate::components::form_items::{JobApplicationFormItem, LeetcodeFormItem};
use crate::components::DynamicForm;
use leptos::*;
use logging;
use serde_json::Value;

#[component]
pub fn Create() -> impl IntoView {
    let data = create_rw_signal(Value::Object(Default::default()));
    view! {
        <div>
            <h1>"Create"</h1>
            <h2>"Leetcode"</h2>
            <DynamicForm<LeetcodeFormItem > data=data/>
            <h2>"Job Application"</h2>
            <DynamicForm<JobApplicationFormItem> data=data/>
        </div>
        <button on:click=move |_| {
            logging::log!("data: {}", data.get().to_string());
        }>"Log Data"</button>
    }
}

