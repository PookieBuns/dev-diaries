use crate::pages::{Home, Login, Create};
use crate::wrappers::{Layout, RequireAuth};
use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=RequireAuth>
                        <Route path="" view=Layout>
                            <Route path="home" view=Home/>
                            <Route path="new" view=Create/>
                            <Route path="" view=|| view! { <h1>"c"</h1> }/>
                        </Route>
                    </Route>
                    <Route path="/login" view=Login/>
                    <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
                </Routes>
            </main>
        </Router>
    }
}

