use leptos::{error::Result, ev::SubmitEvent, *};
use thiserror::Error;
use leptos_router::*;
use crate::utils::base_url;
use crate::wrappers::auth::is_auth;
use crate::components::Alert;
use std::collections::HashMap;

#[derive(Error, Debug)]
pub enum RegisterError {
    #[error("register failed")]
    RegisterFailed,
}

async fn register(username: String, password: String) -> Result<()> {
    let mut map = HashMap::new();
    map.insert("username", username);
    map.insert("password", password);
    let client = reqwest::Client::new();
    let res = client.post(base_url() + "/api/users/register")
        .json(&map)
        .send()
        .await?;
    let response_code = res.status();
    if !response_code.is_success() {
        res.json::<HashMap<String, String>>().await?;
        return Err(RegisterError::RegisterFailed.into());
    }
    logging::log!("response_code: {}", response_code);
    Ok(())
}

#[component]
pub fn Register() -> impl IntoView {
    use leptos::html::Input;
    let user_name: NodeRef<Input> = create_node_ref();
    let password: NodeRef<Input> = create_node_ref();
    let (alert_message, set_alert_message) = create_signal("hi".to_string());
    let (alert_visible, set_alert_visible) = create_signal(false);
    let handle_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let user_name_value = user_name.get().expect("input> to exist").value();
        let password_value = password.get().expect("input> to exist").value();
        logging::log!("user_name_value: {}", user_name_value);
        logging::log!("password_value: {}", password_value);
        let navigate = use_navigate();
        spawn_local(
            async move {
                if register(user_name_value, password_value).await.is_ok() {
                    logging::log!("register success");
                    navigate("/login", Default::default());
                } else {
                    logging::log!("register failed");
                    set_alert_message.set("register failed".to_string());
                    set_alert_visible.set(true);
                }
            }
        );
    };
    let navigate = use_navigate();
    if is_auth() {
        navigate("/home", Default::default());
        view! { <></> }.into_view()
    } else {
        view! {
            <Alert message=alert_message visible=alert_visible/>
            <form on:submit=handle_submit>
                <input required=true type="text" placeholder="Username" node_ref=user_name/>
                <input required=true type="password" placeholder="Password" node_ref=password/>
                <button type="submit">Register</button>
            </form>
            <A href="/login">Login</A>
        }.into_view()
    }
}

















