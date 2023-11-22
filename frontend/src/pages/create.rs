use crate::components::form_items::{JobApplicationFormItem, LeetcodeFormItem};
use crate::components::DynamicForm;
use leptos::*;

#[component]
pub fn Create() -> impl IntoView {
    view! {
        <div>
            <h1>"Create"</h1>
            <h2>"Leetcode"</h2>
            <DynamicForm<LeetcodeFormItem>/>
            <h2>"Job Application"</h2>
            <DynamicForm<JobApplicationFormItem>/>
        </div>
    }
}

