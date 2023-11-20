use crate::components::{Alert, Navbar};
use leptos::*;
use leptos_router::*;

#[component]
pub fn Layout() -> impl IntoView {
    view! {
        <>
            <Navbar/>
            <Outlet/>
        </>
    }
}

