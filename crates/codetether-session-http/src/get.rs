//! Persisted session retrieval.

use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;

/// Load one session by its durable identifier.
pub(super) async fn get<H: crate::SessionHost>(
    Path(id): Path<String>,
) -> Result<Json<H>, (StatusCode, String)> {
    let session = H::load(&id)
        .await
        .map_err(|error| (StatusCode::NOT_FOUND, error.to_string()))?;
    Ok(Json(session))
}
