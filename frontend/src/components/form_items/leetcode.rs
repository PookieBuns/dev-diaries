use crate::components::dynamic_form::{set_checked, set_string, FormItem};
use serde_json::{json, Value};

use leptos::*;

#[derive(Copy, Clone)]
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

    fn data(&self) -> Value {
        json!({
            "id": self.id,
            "link": self.link.get(),
            "difficulty": self.difficulty.get(),
            "is_done": self.is_done.get(),
        })
    }

    fn name() -> &'static str {
        "Leetcode"
    }
}

impl Default for LeetcodeFormItem {
    fn default() -> Self {
        Self {
            id: 0,
            link: RwSignal::new("".to_string()),
            difficulty: RwSignal::new("Medium".to_string()),
            is_done: RwSignal::new(false),
        }
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
                    <option value="Medium" selected>
                        "Medium"
                    </option>
                    <option value="Hard">"Hard"</option>
                </select>
                <input type="checkbox" on:input=set_checked(self.is_done) checked=self.is_done/>
            </>
        }
        .into_view()
    }
}

