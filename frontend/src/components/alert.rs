use leptos::*;

#[component]
pub fn Alert(message: ReadSignal<String>, visible: ReadSignal<bool>) -> impl IntoView {
    move || {
        if visible.get() {
            view! { <div class="alert">{message.get()}</div> }.into_view()
        } else {
            view! { <></> }.into_view()
        }
    }
}

