//! Validate settings without starting a model or confusing configuration with availability.

use super::types::{ApiError, HttpError};
use axum::{Json, http::StatusCode};

pub(super) fn model(value: &str, qualified: bool) -> Result<String, HttpError> {
    let value = value.trim();
    let invalid = value.is_empty()
        || value.len() > 512
        || value
            .chars()
            .any(|ch| ch.is_whitespace() || ch.is_control());
    let qualified_ok = !qualified
        || value.split_once('/').is_some_and(|(provider, model)| {
            !provider.is_empty()
                && !model.is_empty()
                && provider
                    .chars()
                    .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_'))
        });
    if invalid || !qualified_ok {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiError {
                error: "model must be a nonempty provider/model identifier, at most 512 bytes, without whitespace or control characters",
            }),
        ));
    }
    Ok(value.to_string())
}
