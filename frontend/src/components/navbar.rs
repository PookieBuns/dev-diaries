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
        <nav class="navbar bg-body-tertiary navbar-expand-md" data-bs-theme="dark">
            <div class="container-fluid">
                <a class="navbar-brand" href="#">
                    Dev Diaries
                </a>
                <button
                    class="navbar-toggler"
                    type="button"
                    data-bs-toggle="collapse"
                    data-bs-target="#navbarSupportedContent"
                >
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div
                    class="collapse navbar-collapse justify-content-between"
                    id="navbarSupportedContent"
                >
                    <ul class="navbar-nav">
                        <li class="nav-item">
                            <A class="nav-link" href="/home">
                                Home
                            </A>
                        </li>
                        <li class="nav-item">
                            <A class="nav-link" href="/diaries">
                                Diaries
                            </A>
                        </li>
                        <li class="nav-item">
                            <A class="nav-link" href="/new">
                                New Diary
                            </A>
                        </li>
                        <li class="nav-item">
                            <A class="nav-link" href="/">
                                Dummy
                            </A>
                        </li>

                    </ul>
                    <button class="btn btn-outline-danger" on:click=logout>
                        Logout
                    </button>
                </div>
            </div>
        </nav>
    }
}
