use crate::app::AppState;
use crate::auth::Claims;
use crate::models::diary::Diary;
use crate::Result;
use axum::extract::{Extension, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};

pub fn router() -> Router<AppState> {
    Router::new().route("/create", post(create_diary))
}

async fn create_diary(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<Diary>,
) -> Result<impl IntoResponse> {
    println!("{:?}", payload);
    let diary_service = &state.diary_service;
    let diary = Diary {
        user_id: Some(claims.user_id()),
        ..payload
    };
    diary_service.create_diary(diary).await?;
    Ok((StatusCode::OK, "Diary created"))
}
