use leptos::{
    html::{Input, Select},
    *,
};

#[derive(Copy, Clone)]
struct LeetcodeFormItem {
    id: u32,
    link: NodeRef<Input>,
    difficulty: NodeRef<Select>,
    is_done: NodeRef<Input>,
}

#[component]
fn LeetcodeFormItem(form_data: LeetcodeFormItem) -> impl IntoView {
    let link_ref = form_data.link;
    let difficulty_ref = form_data.difficulty;
    let is_done_ref = form_data.is_done;
    view! {
        <>
            <a>{move || form_data.id}</a>
            <input type="text" node_ref=link_ref/>
            <select node_ref=difficulty_ref>
                <option value="Easy">"Easy"</option>
                <option value="Medium">"Medium"</option>
                <option value="Hard">"Hard"</option>
            </select>
            <input type="checkbox" node_ref=is_done_ref/>
        </>
    }
}

#[component]
pub fn LeetcodeForm() -> impl IntoView {
    let (form_items, set_form_items) = create_signal(Vec::<LeetcodeFormItem>::new());
    let (id, set_id) = create_signal(0);
    let (log, set_log) = create_signal(String::new());
    let add_form_item = move |_| {
        let new_item = LeetcodeFormItem {
            id: id.get(),
            link: create_node_ref(),
            difficulty: create_node_ref(),
            is_done: create_node_ref(),
        };
        set_form_items.update(|form_items| form_items.push(new_item));
        set_id.update(|id| *id += 1);
    };
    let submit = move |_| {
        // let mut cur_log = log.get();
        let mut cur_log = String::new();
        cur_log.push_str("Form Submitted:<br>");

        for form_item in form_items.get() {
            let link = form_item.link.get().expect("").value();
            let difficulty = form_item.difficulty.get().expect("").value();
            let is_done = form_item.is_done.get().expect("").checked();
            let res = format!(
                "id: {} link: {}, difficulty: {}, is_done: {}<br>",
                form_item.id, link, difficulty, is_done
            );
            cur_log.push_str(&res);
        }
        cur_log.push_str("<br>");
        set_log.set(cur_log.to_string());
    };
    view! {
        <>
            <div>
                <For each=move || form_items.get() key=|form_item| form_item.id let:form_item>
                    <div>
                        <LeetcodeFormItem form_data=form_item/>
                        <button on:click=move |_| {
                            set_form_items
                                .update(|data| {
                                    data.retain(|item| item.id != form_item.id);
                                });
                        }>"Delete"</button>
                    </div>
                </For>
            </div>
            <button on:click=add_form_item>"Add"</button>
            <button on:click=submit>"Submit"</button>
            <div inner_html=move || log.get()></div>
        </>
    }
}

