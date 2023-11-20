use jsonwebtoken::{decode, DecodingKey, Validation};
use leptos::error::Result;
use leptos::*;
use leptos_router::*;
use serde_json::Value;
use std::collections::HashMap;

pub fn is_auth() -> bool {
    match get_claims() {
        Some(claims) => true,
        None => false,
    }
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
    let navigate = use_navigate();
    if is_auth() {
        view! { <Outlet/> }
    } else {
        navigate("/login", Default::default());
        view! { <></> }.into_view()
    }
}

