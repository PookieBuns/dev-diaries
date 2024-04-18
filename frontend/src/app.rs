use crate::pages::{Create, Diaries, Home, Login, Register, Today};
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
                            <Route path="diaries" view=Diaries/>
                            <Route path="new" view=Create/>
                            <Route path="today" view=Today/>
                            <Route path="" view=|| view! { <h1>There is nothing here</h1> }/>
                        </Route>
                    </Route>
                    <Route path="/login" view=Login/>
                    <Route path="/register" view=Register/>
                    <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
                </Routes>
            </main>
        </Router>
    }
}
