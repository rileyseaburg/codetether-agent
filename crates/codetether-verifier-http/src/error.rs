//! Map configuration failures without exposing storage paths or credentials.
use super::types::{ApiError, HttpError};
use axum::{Json, http::StatusCode};

pub(super) fn configuration(_: anyhow::Error) -> HttpError {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ApiError {
            error: "verifier configuration unavailable",
        }),
    )
}
