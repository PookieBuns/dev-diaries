use crate::components::Navbar;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Layout() -> impl IntoView {
    view! {
        <>
            <Navbar/>
            <div class="container">
                <Outlet/>
            </div>
        </>
    }
}
