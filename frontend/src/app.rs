use crate::wrappers::auth::RequireAuth;
use crate::pages::home::Home;
use crate::pages::login::Login;
use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=RequireAuth>
                        <Route path="home" view=Home/>
                        <Route path="" view=|| view! { <h1>"c"</h1> }/>
                    </Route>
                    <Route path="/login" view=Login/>
                    <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
                </Routes>
            </main>
        </Router>
    }
}

