//! Persisted session retrieval.

use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;

/// Load one session by its durable identifier.
pub(super) async fn get(
    Path(id): Path<String>,
) -> Result<Json<crate::session::Session>, (StatusCode, String)> {
    let session = crate::session::Session::load(&id)
        .await
        .map_err(|error| (StatusCode::NOT_FOUND, error.to_string()))?;
    Ok(Json(session))
}
