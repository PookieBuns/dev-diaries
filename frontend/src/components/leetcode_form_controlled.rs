use leptos::ev::Event;
use leptos::*;

fn set_string(signal: RwSignal<String>) -> impl Fn(Event) {
    move |e| signal.set(event_target_value(&e))
}

fn set_checked(signal: RwSignal<bool>) -> impl Fn(Event) {
    move |e| signal.set(event_target_checked(&e))
}

#[derive(Copy, Clone)]
struct LeetcodeFormItem {
    id: u32,
    link: RwSignal<String>,
    difficulty: RwSignal<String>,
    is_done: RwSignal<bool>,
}

#[component]
fn LeetcodeFormItem(form_data: LeetcodeFormItem) -> impl IntoView {
    view! {
        <>
            <a>{form_data.id}</a>
            <input
                type="text"
                name="link"
                on:input=set_string(form_data.link)
                prop:value=form_data.link
            />
            <select on:input=set_string(form_data.difficulty) prop:value=form_data.difficulty>
                <option value="Easy">"Easy"</option>
                <option value="Medium">"Medium"</option>
                <option value="Hard">"Hard"</option>
            </select>
            <input
                type="checkbox"
                on:input=set_checked(form_data.is_done)
                checked=form_data.is_done
            />
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
            link: create_rw_signal(String::new()),
            difficulty: create_rw_signal("Easy".to_string()),
            is_done: create_rw_signal(false),
        };
        set_form_items.update(|form_items| form_items.push(new_item));
        set_id.update(|id| *id += 1);
    };
    let submit = move |_| {
        // let mut cur_log = log.get();
        let mut cur_log = String::new();
        cur_log.push_str("Form Submitted:<br>");

        for form_item in form_items.get() {
            let link = form_item.link.get();
            let difficulty = form_item.difficulty.get();
            let is_done = form_item.is_done.get();
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
                {id} <For each=move || form_items.get() key=|form_item| form_item.id let:form_item>
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

