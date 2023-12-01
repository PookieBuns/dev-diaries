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
            <A href="/home">Home</A>
            <A href="/new">New Diary</A>
            <A href="/">Dummy</A>
            <button on:click=logout>Logout</button>
        </div>
    }
}

