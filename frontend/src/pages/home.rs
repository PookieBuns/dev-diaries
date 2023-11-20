use crate::wrappers::auth::get_claims;
use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    let claims = get_claims().unwrap_or_default();
    let curr_time = chrono::Utc::now().timestamp();
    view! {
        <div>
            <h1>Home</h1>
            <p>Welcome to the home page!</p>
            <p>Current time: {curr_time}</p>
            <p>Here are your claims from your jwt:</p>
            <ul>
                {claims
                    .into_iter()
                    .map(|(k, v)| view! { <li>{k} : {v.to_string()}</li> })
                    .collect_view()}
            </ul>
        </div>
    }
}

