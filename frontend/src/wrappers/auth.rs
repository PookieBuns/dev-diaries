use jsonwebtoken::{decode, DecodingKey, Validation};
use leptos::error::Result;
use leptos::*;
use leptos_router::*;
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;

pub fn is_auth() -> bool {
    get_claims().is_some()
}

fn decode_jwt(token: &str) -> Result<HashMap<String, Value>> {
    let dummy_key = DecodingKey::from_secret("".as_ref());
    let mut validation = Validation::default();
    validation.insecure_disable_signature_validation();
    validation.leeway = 0;
    let decoded = decode::<HashMap<String, Value>>(token, &dummy_key, &validation)?;
    Ok(decoded.claims)
}

pub fn get_jwt() -> Option<String> {
    wasm_cookies::get_raw("auth-token")
}

pub fn get_claims() -> Option<HashMap<String, Value>> {
    match get_jwt() {
        Some(token) => match decode_jwt(&token) {
            Ok(claims) => Some(claims),
            Err(_) => None,
        },
        None => None,
    }
}

#[component]
pub fn RequireAuth() -> impl IntoView {
    let check_auth = move || {
        logging::log!("Checking if user is authenticated");
        if !is_auth() {
            let navigate = use_navigate();
            navigate("/login", Default::default());
        }
    };
    let interval_handle = set_interval_with_handle(check_auth, Duration::from_secs(5)).unwrap();
    on_cleanup(move || {
        logging::log!("Clearing interval");
        interval_handle.clear();
    });
    if is_auth() {
        logging::log!("User is authenticated");
        view! { <Outlet/> }
    } else {
        let navigate = use_navigate();
        navigate("/login", Default::default());
        view! { <></> }.into_view()
    }
}
