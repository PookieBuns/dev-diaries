use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::{Response, IntoResponse};
use tower_cookies::Cookies;

use crate::routes::AUTH_TOKEN;

pub async fn mw_require_auth<B>(
    cookies: Cookies,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, Response> {
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
    println!("auth_token: {:?}", auth_token);
    auth_token.ok_or((StatusCode::UNAUTHORIZED, "UNAUTHORIZED").into_response())?;
    Ok(next.run(req).await)
}
