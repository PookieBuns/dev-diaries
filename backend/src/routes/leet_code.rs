use crate::app::AppState;
use crate::Result;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde::Deserialize;

#[derive(Deserialize)]
struct QueryParams {
    session_token: String,
}

pub fn router() -> Router<AppState> {
    Router::new().route("/submissions", get(get_submissions))
}

async fn get_submissions(
    Query(query): Query<QueryParams>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    let leet_code_service = &state.leet_code_service;
    let submissions = leet_code_service
        .get_submissions(&query.session_token, 0, 20)
        .await?;
    Ok((StatusCode::OK, Json(submissions)))
}
