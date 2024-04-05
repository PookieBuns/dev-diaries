use crate::app::AppState;
use crate::model::Diary;
use crate::Result;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use serde_json::Value;

pub fn router() -> Router<AppState> {
    Router::new().route("/create", post(create_diary))
}

async fn create_diary(
    State(state): State<AppState>,
    Json(payload): Json<Diary>,
) -> Result<impl IntoResponse> {
    println!("{:?}", payload);
    let diary_service = &state.diary_service;
    diary_service.create_diary(payload).await?;
    Ok((StatusCode::OK, "Diary created"))
}
