use crate::components::dynamic_form::{set_checked, set_string, FormItem};
use serde::{Deserialize, Serialize};

use leptos::*;

#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct LeetcodeFormItem {
    #[serde(rename = "leet_code_problem_id")]
    id: u32,
    problem_link: RwSignal<String>,
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

    fn name() -> &'static str {
        "Leetcode"
    }
}

impl Default for LeetcodeFormItem {
    fn default() -> Self {
        Self {
            id: 0,
            problem_link: RwSignal::new("".to_string()),
            difficulty: RwSignal::new("Medium".to_string()),
            is_done: RwSignal::new(false),
        }
    }
}

impl IntoView for LeetcodeFormItem {
    fn into_view(self) -> View {
        view! {
            <div class="col">
                <input
                    class="form-control"
                    required
                    type="text"
                    placeholder="link"
                    on:input=set_string(self.problem_link)
                    prop:value=self.problem_link
                />
            </div>
            <div class="col">
                <select
                    class="form-select"
                    on:input=set_string(self.difficulty)
                    prop:value=self.difficulty
                >
                    <option value="Easy">"Easy"</option>
                    <option value="Medium" selected>
                        "Medium"
                    </option>
                    <option value="Hard">"Hard"</option>
                </select>
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
