use leptos::{error::Result, ev::SubmitEvent, *};
use leptos_router::use_navigate;
use crate::storage;

async fn login(username: String, password: String) -> Result<()> {
    let res = reqwasm::http::Request::get("http://localhost:3000/")
        .send()
        .await?.text().await?;
    logging::log!("res: {}", res);
    let local_storage = storage::get_local_storage()?;
    local_storage.set_item("token", "123");
    Ok(())
}

#[component]
pub fn Login() -> impl IntoView {
    use leptos::html::Input;
    let user_name: NodeRef<Input> = create_node_ref();
    let password: NodeRef<Input> = create_node_ref();
    let handle_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let user_name_value = user_name.get().expect("input> to exist").value();
        let password_value = password.get().expect("input> to exist").value();
        logging::log!("user_name_value: {}", user_name_value);
        logging::log!("password_value: {}", password_value);
        let navigate = use_navigate();
        spawn_local(
            async move {
                if login(user_name_value, password_value).await.is_ok() {
                    logging::log!("login success");
                    navigate("/home", Default::default());
                } else {
                    logging::log!("login failed");
                }
            }
        );
    };
    view! {
        <form on:submit=handle_submit>
            <input type="text" placeholder="Username" node_ref=user_name/>
            <input type="password" placeholder="Password" node_ref=password/>
            <button type="submit">Login</button>
        </form>
    }
}




