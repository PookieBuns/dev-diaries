use crate::components::form_items::{JobApplicationFormItem, LeetcodeFormItem};
use crate::components::DynamicForm;
use crate::components::MarkdownInput;
use leptos::ev::SubmitEvent;
use leptos::*;

pub struct Form {
    pub id: RwSignal<Option<u64>>,
    pub leetcode: RwSignal<Vec<LeetcodeFormItem>>,
    pub job_application: RwSignal<Vec<JobApplicationFormItem>>,
    pub notes: RwSignal<String>,
}

#[component]
pub fn DiaryForm<F>(form_data: Form, handle_submit: F) -> impl IntoView
where
    F: Fn(SubmitEvent) + 'static,
{
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
