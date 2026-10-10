//! Bounded JSON body reading without logging or persisting request bodies.
use crate::ApiError;
use axum::body::Body;
use axum::http::{HeaderMap, header};
use serde_json::Value;
use std::time::Duration;

/// Default owner/pairing body limit, matching the TypeScript relay.
pub(crate) const SMALL: usize = 4096;
/// Frame upload body limit.
pub(crate) const FRAME: usize = 710_000;

/// Require `application/json`, enforce `limit`, and parse the body.
///
/// # Errors
/// 415 for other content types, 413 when too large, 408 on a slow body,
/// and 400 for malformed JSON.
pub(crate) async fn read_json(
    headers: &HeaderMap,
    body: Body,
    limit: usize,
) -> Result<Value, ApiError> {
    let kind = headers
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.split(';').next())
        .map(str::trim);
    if kind != Some("application/json") {
        return Err(ApiError::new(415, "Use application/json"));
    }
    let declared = headers
        .get(header::CONTENT_LENGTH)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<usize>().ok());
    if declared.is_some_and(|length| length > limit) {
        return Err(ApiError::new(413, "Request too large"));
    }
    let read = axum::body::to_bytes(body, limit);
    let bytes = tokio::time::timeout(Duration::from_secs(15), read)
        .await
        .map_err(|_| ApiError::new(408, "Request timed out"))?
        .map_err(|_| ApiError::new(413, "Request too large"))?;
    serde_json::from_slice(&bytes).map_err(|_| ApiError::new(400, "Invalid JSON"))
}
