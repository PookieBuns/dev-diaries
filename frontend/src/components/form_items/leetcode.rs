use crate::components::dynamic_form::{set_checked, set_string, FormItem};

use leptos::*;

#[derive(Copy, Clone, Default)]
pub struct LeetcodeFormItem {
    id: u32,
    link: RwSignal<String>,
    difficulty: RwSignal<String>,
    is_done: RwSignal<bool>,
}

impl FormItem for LeetcodeFormItem {
    fn id(&self) -> u32 {
        self.id
    }

    fn set_id(&mut self, id: u32) {
        self.id = id;
    }
}

impl IntoView for LeetcodeFormItem {
    fn into_view(self) -> View {
        view! {
            <>
                <a>{self.id}</a>
                <input
                    type="text"
                    placeholder="link"
                    on:input=set_string(self.link)
                    prop:value=self.link
                />
                <select on:input=set_string(self.difficulty) prop:value=self.difficulty>
                    <option value="Easy">"Easy"</option>
                    <option value="Medium">"Medium"</option>
                    <option value="Hard">"Hard"</option>
                </select>
                <input type="checkbox" on:input=set_checked(self.is_done) checked=self.is_done/>
            </>
        }
        .into_view()
    }
}

