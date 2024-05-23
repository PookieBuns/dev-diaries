use crate::app::AppState;
use crate::auth::Claims;
use crate::Result;
use axum::extract::{Extension, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde::Deserialize;

#[derive(Deserialize)]
struct QueryParams {
    session_token: String,
    offset: Option<i32>,
    limit: Option<i32>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/submissions", get(get_submissions))
        .route("/submissions/all", get(get_all_submissions))
        .route("/submissions/sync", get(sync_submissions))
}

async fn get_submissions(
    Query(query): Query<QueryParams>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    let leet_code_service = &state.leet_code_service;
    let submissions = leet_code_service
        .get_submissions(
            &query.session_token,
            query.offset.unwrap_or(0),
            query.limit.unwrap_or(10),
        )
        .await?;
    Ok((StatusCode::OK, Json(submissions)))
}

async fn get_all_submissions(
    Query(query): Query<QueryParams>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    let leet_code_service = &state.leet_code_service;
    let submissions = leet_code_service
        .get_all_submissions(&query.session_token)
        .await?;
    Ok((StatusCode::OK, Json(submissions)))
}

async fn sync_submissions(
    Query(query): Query<QueryParams>,
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse> {
    let leet_code_service = &state.leet_code_service;
    leet_code_service
        .sync_submissions(claims.user_id(), &query.session_token)
        .await?;
    Ok((StatusCode::OK, "OK"))
}
