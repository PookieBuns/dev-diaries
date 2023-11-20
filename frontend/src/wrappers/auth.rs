use jsonwebtoken::{decode, DecodingKey, Validation};
use leptos::*;
use leptos_router::*;
use serde_json::Value;
use std::collections::HashMap;

pub fn is_auth() -> bool {
    match wasm_cookies::get_raw("auth-token") {
        Some(_) => true,
        None => false,
    }
}

fn decode_jwt(token: &str) -> HashMap<String, Value> {
    let dummy_key = DecodingKey::from_secret("".as_ref());
    let mut no_validation = Validation::default();
    no_validation.insecure_disable_signature_validation();
    let decoded = decode::<HashMap<String, Value>>(token, &dummy_key, &no_validation).unwrap();
    decoded.claims
}

pub fn get_jwt() -> Option<String> {
    wasm_cookies::get_raw("auth-token")
}

pub fn get_claims() -> Option<HashMap<String, Value>> {
    match get_jwt() {
        Some(token) => Some(decode_jwt(&token)),
        None => None,
    }
}

#[component]
pub fn RequireAuth() -> impl IntoView {
    let navigate = use_navigate();
    if is_auth() {
        view! { <Outlet/> }
    } else {
        navigate("/login", Default::default());
        view! { <></> }.into_view()
    }
}

