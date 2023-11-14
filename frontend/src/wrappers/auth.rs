use leptos::*;
use leptos_router::*;
use crate::storage;

pub fn is_auth() -> bool {
    let local_storage = storage::get_local_storage().expect("local_storage to exist");
    let token = local_storage.get_item("token").expect("token to exist");
    if token.is_some() && token.unwrap() == "123" {
        true
    } else {
        false
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

