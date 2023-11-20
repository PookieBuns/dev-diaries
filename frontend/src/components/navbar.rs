use leptos::*;
use leptos_router::*;

#[component]
pub fn Navbar() -> impl IntoView {
    let logout = move |_| {
        wasm_cookies::delete_raw("auth-token");
        let navigate = use_navigate();
        navigate("/login", Default::default());
    };
    view! {
        <div class="navbar">
            <a href="/home">Home</a>
            <a href="/new">New Diary</a>
            <a href="/">Dummy</a>
            <button on:click=logout>Logout</button>
        </div>
    }
}

