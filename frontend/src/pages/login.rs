use leptos::ev::SubmitEvent;
use leptos::{leptos_dom::logging::console_log, *};

#[component]
pub fn Login() -> impl IntoView {
    let handle_submit = |ev: SubmitEvent| {
        ev.prevent_default();
        console_log("Submitted!");
    };
    view! {
        <form on:submit=handle_submit>
            <input type="text" name="username" placeholder="Username"/>
            <input type="password" name="password" placeholder="Password"/>
            <button type="submit">Login</button>
        </form>
    }
}

