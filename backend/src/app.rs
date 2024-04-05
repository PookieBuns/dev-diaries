use crate::db;
use crate::middleware::mw_require_auth;
use crate::repository::diary_repository::PgDiaryRepo;
use crate::repository::token_repository::MemTokenRepo;
use crate::repository::user_repository::PgUserRepo;
use crate::routes::diary::router as diary_router;
use crate::routes::users::router as users_router;
use crate::service::DiaryService;
use crate::service::UserService;
use axum::http::StatusCode;
use axum::middleware;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use tower_cookies::{CookieManagerLayer, Cookies};
use tower_http::cors::{Any, CorsLayer};

// Use this to configure the type of the user repository.
type UserRepoImpl = PgUserRepo;
type TokenRepoImpl = MemTokenRepo;
type DiaryRepoImpl = PgDiaryRepo;

#[derive(Clone)]
pub struct AppState {
    pub user_service: UserService<UserRepoImpl, TokenRepoImpl>,
    pub diary_service: DiaryService<DiaryRepoImpl>,
}

fn api_router() -> Router<AppState> {
    Router::new()
        .route("/cookies", get(read_cookies))
        .nest("/diary", diary_router())
        .layer(middleware::from_fn(mw_require_auth))
        .nest("/users", users_router())
}

pub async fn app() -> Router {
    let pool = db::db_pool().await.unwrap();
    let user_repo = UserRepoImpl::new(pool.clone());
    let token_repo = TokenRepoImpl::new();
    let diary_repo = PgDiaryRepo::new(pool.clone());
    let user_service = UserService::new(user_repo, token_repo);
    let diary_service = DiaryService::new(diary_repo);
    let app_state = AppState {
        user_service,
        diary_service,
    };
    Router::new()
        .route("/", get(probe))
        .nest("/api", api_router())
        .with_state(app_state)
        .layer(CookieManagerLayer::new())
        .layer(CorsLayer::new().allow_origin(Any).allow_headers(Any))
        .fallback(not_found)
}

async fn probe() -> &'static str {
    "OK"
}

async fn read_cookies(cookies: Cookies) -> String {
    let mut cookies_str = String::new();
    for cookie in cookies.list() {
        cookies_str.push_str(&format!("{}: {}\n", cookie.name(), cookie.value()));
    }
    cookies_str
}

async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not Found")
}
