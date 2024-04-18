use crate::api::diary::create_diary;
use crate::components::diary_form::Form;
use crate::components::DiaryForm;
use leptos::ev::SubmitEvent;
use leptos::*;
use leptos_router::*;
use serde_json::{json, Value};

#[component]
pub fn Create() -> impl IntoView {
    let form_data = Form {
        id: RwSignal::new(None),
        leetcode: RwSignal::new(vec![]),
        job_application: RwSignal::new(vec![]),
        notes: RwSignal::new("".to_string()),
    };
    let save_diary = create_action(|(json_data, redirect): &(Value, bool)| {
        let navigate = use_navigate();
        let redirect = *redirect;
        let json_data = json_data.clone();
        async move {
            let result = create_diary(json_data).await;
            if redirect && result.is_ok() {
                navigate("/home", Default::default());
            }
            result
        }
    });
    let get_diary_data = move || {
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
        json_data["diary_id"] = json!(form_data.id.get());
        json_data
    };
    let handle_submit = move |ev: SubmitEvent| {
        logging::log!("submit");
        ev.prevent_default();
        let json_data = get_diary_data();
        save_diary.dispatch((json_data, true));
    };
    view! {
        <h1>Create</h1>
        <DiaryForm form_data=form_data handle_submit=handle_submit/>
    }
}
