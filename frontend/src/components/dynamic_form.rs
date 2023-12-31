use leptos::{ev::Event, *};

pub fn set_string(signal: RwSignal<String>) -> impl Fn(Event) {
    move |e| signal.set(event_target_value(&e))
}

pub fn set_checked(signal: RwSignal<bool>) -> impl Fn(Event) {
    move |e| signal.set(event_target_checked(&e))
}

pub trait FormItem: IntoView + Default + Clone + Copy {
    fn id(&self) -> u32;
    fn set_id(&mut self, id: u32);
}

#[component]
pub fn DynamicForm<T: FormItem + 'static>(#[prop(optional)] _dummy: T) -> impl IntoView {
    let (form_items, set_form_items) = create_signal(Vec::<T>::new());
    let (id, set_id) = create_signal(0);
    let (log, set_log) = create_signal(String::new());
    let add_form_item = move |_| {
        let mut new_item = T::default();
        new_item.set_id(id.get());
        set_form_items.update(|form_items| form_items.push(new_item));
        set_id.update(|id| *id += 1);
    };
    let submit = move |_| {
        // let mut cur_log = log.get();
        let mut cur_log = String::new();
        cur_log.push_str("Form Submitted:<br>");

        cur_log.push_str("<br>");
        set_log.set(cur_log.to_string());
    };
    view! {
        <>
            <div>
                {id}
                <For each=move || form_items.get() key=|form_item| form_item.id() let:form_item>
                    <div>
                        {form_item}
                        <button on:click=move |_| {
                            set_form_items
                                .update(|data| {
                                    data.retain(|item| item.id() != form_item.id());
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

