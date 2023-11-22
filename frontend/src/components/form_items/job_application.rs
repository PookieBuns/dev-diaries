use crate::components::dynamic_form::{set_checked, set_string, FormItem};

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

