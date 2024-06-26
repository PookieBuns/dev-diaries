use crate::api::diary::{create_diary, get_diaries};
use crate::components::diary_form::Form;
use crate::components::form_items::{JobApplicationFormItem, LeetcodeFormItem};
use crate::components::DiaryForm;
use leptos::ev::SubmitEvent;
use leptos::*;
use leptos_router::*;
use serde_json::{json, Value};
use std::time::Duration;

#[component]
pub fn Today() -> impl IntoView {
    let form_signal = create_local_resource(
        || (),
        |_| async move {
            let diaries = get_diaries().await;
            let form = Form {
                leetcode: create_rw_signal(vec![]),
                job_application: create_rw_signal(vec![]),
                notes: create_rw_signal("".to_string()),
                id: create_rw_signal(None),
            };
            if let Ok(diaries) = diaries {
                logging::log!("{}", diaries.to_string());
                let diaries = diaries.as_array().unwrap();
                let today = chrono::Local::now().date_naive().to_string();
                let today_diary = diaries
                    .iter()
                    .find(|diary| diary["diary_date"].as_str().unwrap() == today);
                if let Some(today_diary) = today_diary {
                    let leet_code_problems: Vec<LeetcodeFormItem> =
                        serde_json::from_value(today_diary["leet_code_problems"].clone()).unwrap();
                    form.leetcode.set(leet_code_problems);
                    let job_applications: Vec<JobApplicationFormItem> =
                        serde_json::from_value(today_diary["job_applications"].clone()).unwrap();
                    form.job_application.set(job_applications);
                    let notes = today_diary["diary_notes"].as_str().unwrap();
                    form.notes.set(notes.to_string());
                    form.id.set(Some(today_diary["diary_id"].as_u64().unwrap()));
                }
            }
            form
        },
    );
    let last_update_time =
        RwSignal::new(chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
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
        form_signal.with(|maybe_form_data| {
            if let Some(form_data) = maybe_form_data {
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
            }
        });
        json_data
    };
    let handle_submit = move |ev: SubmitEvent| {
        logging::log!("submit");
        ev.prevent_default();
        let json_data = get_diary_data();
        save_diary.dispatch((json_data, false));
    };
    create_effect(move |_| {
        let save_diary_result = save_diary.value().get();
        if let Some(Ok(res_json)) = save_diary_result {
            last_update_time.set(chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
            form_signal.with(|maybe_form_data| {
                if let Some(form_data) = maybe_form_data {
                    form_data
                        .id
                        .set(Some(res_json["diary_id"].as_u64().unwrap()));
                }
            })
        }
    });
    let interval_handle = set_interval_with_handle(
        move || {
            logging::log!("Auto saving diary");
            let json_data = get_diary_data();
            save_diary.dispatch((json_data, false));
        },
        Duration::from_secs(10),
    )
    .unwrap();
    on_cleanup(move || {
        logging::log!("Clearing auto save interval");
        interval_handle.clear();
    });

    view! {
        <h1>Today</h1>
        <p>"Last updated: " {last_update_time}</p>
        <Suspense fallback=move || {
            view! { <p>"Loading..."</p> }
        }>
            {move || {
                form_signal.with(|maybe_form_data| {
                    maybe_form_data.map(|form_data| {
                        view! {
                            <DiaryForm form_data=form_data handle_submit=handle_submit/>
                        }
                })
            })}}

        </Suspense>
    }
}
