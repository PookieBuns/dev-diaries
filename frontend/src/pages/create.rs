use crate::components::LeetcodeForm;
use leptos::*;

#[component]
pub fn Create() -> impl IntoView {
    view! {
        <div>
            <h1>"Create"</h1>
            <LeetcodeForm />
        </div>
    }
}
