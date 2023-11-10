use leptos::*;
use leptos_router::*;
use crate::storage;

#[component]
pub fn Home() -> impl IntoView {
    let logout = move |_| {
        let local_storage = storage::get_local_storage().expect("local_storage to exist");
        local_storage.remove_item("token").expect("token to exist");
        let navigate = use_navigate();
        navigate("/login", Default::default());
    };
    view! {
        <div>
            <h1>Home</h1>
            <p>Welcome to the home page!</p>
            <button on:click=logout>Logout</button>
        </div>
    }
}

