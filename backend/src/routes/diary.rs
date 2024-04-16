use crate::app::AppState;
use crate::auth::Claims;
use crate::models::diary::Diary;
use crate::Result;
use axum::extract::{Extension, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_diary))
        .route("/get", get(get_diaries))
}

async fn create_diary(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<Diary>,
) -> Result<impl IntoResponse> {
    let diary_service = &state.diary_service;
    let diary = Diary {
        user_id: Some(claims.user_id()),
        ..payload
    };
    diary_service.create_diary(diary).await?;
    Ok((StatusCode::OK, "Diary created"))
}

async fn get_diaries(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse> {
    let diary_service = &state.diary_service;
    let diaries = diary_service.get_diaries(claims.user_id()).await?;
    Ok((StatusCode::OK, Json(diaries)))
}
