use crate::db;
use crate::middleware::mw_require_auth;
use crate::repository::diary_repository::PgDiaryRepo;
use crate::repository::leet_code_repository::HttpLeetCodeRepo;
use crate::repository::token_repository::MemTokenRepo;
use crate::repository::user_repository::PgUserRepo;
use crate::routes::diary::router as diary_router;
use crate::routes::leet_code::router as leet_code_router;
use crate::routes::users::router as users_router;
use crate::service::DiaryService;
use crate::service::LeetCodeService;
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
type LeetCodeRepoImpl = HttpLeetCodeRepo;

#[derive(Clone)]
pub struct AppState {
    pub user_service: UserService<UserRepoImpl, TokenRepoImpl>,
    pub diary_service: DiaryService<DiaryRepoImpl>,
    pub leet_code_service: LeetCodeService<LeetCodeRepoImpl, DiaryRepoImpl>,
}

fn api_router() -> Router<AppState> {
    Router::new()
        .route("/cookies", get(read_cookies))
        .nest("/diary", diary_router())
        .nest("/leet-code", leet_code_router())
        .layer(middleware::from_fn(mw_require_auth))
        .route("/ping", get(probe))
        .nest("/users", users_router())
}

pub async fn app() -> Router {
    let pool = db::db_pool().await.unwrap();
    let user_repo = UserRepoImpl::new(pool.clone());
    let token_repo = TokenRepoImpl::new();
    let diary_repo = PgDiaryRepo::new(pool.clone());
    let leet_code_repo = HttpLeetCodeRepo::new();
    let user_service = UserService::new(user_repo, token_repo);
    let diary_service = DiaryService::new(diary_repo.clone());
    let leet_code_service = LeetCodeService::new(leet_code_repo, diary_repo.clone());
    let app_state = AppState {
        user_service,
        diary_service,
        leet_code_service,
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
    println!("ping");
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
