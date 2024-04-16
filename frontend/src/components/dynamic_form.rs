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
    fn name() -> &'static str;
}

#[component]
pub fn DynamicForm<T: FormItem + 'static>(form_items: RwSignal<Vec<T>>) -> impl IntoView {
    let (id, set_id) = create_signal(0);
    let add_form_item = move |_| {
        let mut new_item = T::default();
        new_item.set_id(id.get());
        form_items.update(|form_items| form_items.push(new_item));
        set_id.update(|id| *id += 1);
    };
    view! {
        <div class="row">
            <div class="col">
                <For each=move || form_items.get() key=|form_item| form_item.id() let:form_item>
                    <div class="row mb-3">
                        {form_item} <div class="col">
                            <button
                                type="button"
                                class="btn btn-danger"
                                on:click=move |_| {
                                    logging::log!("delete form item: {}", form_item.id());
                                    form_items
                                        .update(|data| {
                                            data.retain(|item| item.id() != form_item.id());
                                        });
                                }
                            >

                                Delete
                            </button>
                        </div>
                    </div>
                </For>
            </div>
        </div>
        <div class="d-grid mb-3">
            <button type="button" class="btn btn-secondary" on:click=add_form_item>
                Add
            </button>
        </div>
    }
}
