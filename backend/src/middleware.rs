use crate::auth::{decode_jwt, AUTH_TOKEN};
use crate::errors::Error;
use crate::Result;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use tower_cookies::Cookies;

pub async fn mw_require_auth<B>(
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
    let jwt = auth_token.ok_or(Error::Auth)?;
    let claims = decode_jwt(&jwt)?;
    req.extensions_mut().insert(claims);
    Ok(next.run(req).await)
}
