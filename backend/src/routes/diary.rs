use crate::app::AppState;
use crate::auth::Claims;
use crate::models::diary::Diary;
use crate::Result;
use axum::extract::{Extension, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde_json::json;

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
    println!("Creating diary: {:?}", payload);
    let diary_service = &state.diary_service;
    let diary_id = diary_service
        .create_diary(claims.user_id(), payload)
        .await?;
    let diary_response = json!({"diary_id": diary_id});
    Ok((StatusCode::OK, Json(diary_response)))
}

async fn get_diaries(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse> {
    let diary_service = &state.diary_service;
    let diaries = diary_service.get_diaries(claims.user_id()).await?;
    Ok((StatusCode::OK, Json(diaries)))
}
