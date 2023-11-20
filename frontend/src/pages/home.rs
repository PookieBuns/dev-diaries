use crate::wrappers::auth::get_claims;
use leptos::*;
use leptos_router::*;
use std::collections::HashMap;
use wasm_cookies;

#[component]
pub fn Home() -> impl IntoView {
    let logout = move |_| {
        wasm_cookies::delete_raw("auth-token");
        let navigate = use_navigate();
        navigate("/login", Default::default());
    };
    let claims = get_claims().unwrap_or(HashMap::new());
    for (key, value) in &claims {
        logging::log!("{}: {}", key, value);
    }
    view! {
        <div>
            <h1>Home</h1>
            <p>Welcome to the home page!</p>
            <p>Here are your claims from your jwt:</p>
            <ul>
                {claims
                    .into_iter()
                    .map(|(key, value)| {
                        view! { <li>{key} : {value.to_string()}</li> }
                    })
                    .collect::<Vec<_>>()}
            </ul>
            <button on:click=logout>Logout</button>
        </div>
    }
}

