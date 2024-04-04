use crate::components::dynamic_form::{set_checked, set_string, FormItem};
use serde_json::json;

use leptos::*;

#[derive(Copy, Clone, Default)]
pub struct JobApplicationFormItem {
    id: u32,
    company_name: RwSignal<String>,
    link: RwSignal<String>,
    is_done: RwSignal<bool>,
}

impl FormItem for JobApplicationFormItem {
    fn id(&self) -> u32 {
        self.id
    }

    fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    fn data(&self) -> String {
        json!({
            "id": self.id,
            "company_name": self.company_name.get(),
            "link": self.link.get(),
            "is_done": self.is_done.get(),
        })
        .to_string()
    }
}

impl IntoView for JobApplicationFormItem {
    fn into_view(self) -> View {
        view! {
            <>
                <a>{self.id}</a>
                <input
                    type="text"
                    placeholder="company name"
                    on:input=set_string(self.company_name)
                    prop:value=self.company_name
                />
                <input
                    type="text"
                    placeholder="link"
                    on:input=set_string(self.link)
                    prop:value=self.link
                />
                <input type="checkbox" on:input=set_checked(self.is_done) checked=self.is_done/>
            </>
        }
        .into_view()
    }
}

