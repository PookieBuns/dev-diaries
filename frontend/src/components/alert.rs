use leptos::*;

#[component]
pub fn Alert(message: ReadSignal<String>, visible: ReadSignal<bool>) -> impl IntoView {
    view! {
        <Show when=move || visible.get()>
            <div class="alert">{message}</div>
        </Show>
    }
}
