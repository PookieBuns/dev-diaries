use leptos::*;
use leptos_router::*;
use crate::storage;

#[component]
pub fn RequireAuth() -> impl IntoView {
    let local_storage = storage::get_local_storage().expect("local_storage to exist");
    let token = local_storage.get_item("token").expect("token to exist");
    let navigate = use_navigate();
    if token.is_some() && token.unwrap() == "123" {
        view! { <Outlet/> }
    } else {
        navigate("/login", Default::default());
        view! { <></> }.into_view()
    }
}

