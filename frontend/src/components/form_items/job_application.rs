use crate::components::dynamic_form::{set_checked, set_string, FormItem};
use serde::Deserialize;
use serde_json::{json, Value};

use leptos::*;

#[derive(Copy, Clone, Default, Deserialize)]
pub struct JobApplicationFormItem {
    #[serde(alias = "job_application_id")]
    id: u32,
    company_name: RwSignal<String>,
    job_application_link: RwSignal<String>,
    is_done: RwSignal<bool>,
}

impl FormItem for JobApplicationFormItem {
    fn id(&self) -> u32 {
        self.id
    }

    fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    fn data(&self) -> Value {
        json!({
            "id": self.id,
            "company_name": self.company_name.get(),
            "job_application_link": self.job_application_link.get(),
            "is_done": self.is_done.get(),
        })
    }

    fn name() -> &'static str {
        "Job Application"
    }
}

impl IntoView for JobApplicationFormItem {
    fn into_view(self) -> View {
        view! {
            <div class="col">
                // <a>{self.id}</a>
                <input
                    class="form-control"
                    required
                    type="text"
                    placeholder="company name"
                    on:input=set_string(self.company_name)
                    prop:value=self.company_name
                />
            </div>
            <div class="col">
                <input
                    class="form-control"
                    type="text"
                    placeholder="link"
                    on:input=set_string(self.job_application_link)
                    prop:value=self.job_application_link
                />
            </div>
            <div class="col">
                <div class="form-check">
                    <input
                        class="form-check-input"
                        type="checkbox"
                        on:input=set_checked(self.is_done)
                        checked=self.is_done
                    />
                    <label class="form-check-label">"Done"</label>
                </div>
            </div>
        }
        .into_view()
    }
}
