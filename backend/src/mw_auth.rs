use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use tower_cookies::Cookies;

use crate::error::{Error, Result};
use crate::routes::AUTH_TOKEN;

pub async fn mw_require_auth<B>(
    cookies: Cookies,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
    println!("auth_token: {:?}", auth_token);
    auth_token.ok_or(Error::AuthFailNoAuthTokenCookie)?;
    Ok(next.run(req).await)
}
