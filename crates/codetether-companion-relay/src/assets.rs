//! Fixed-allowlist web shell; never serves a request-selected path.
use crate::{ApiError, assets_headers::headers};
use axum::response::{IntoResponse, Response};
use std::path::Path;

const FILES: &[&str] = &[
    "index.html",
    "style.css",
    "app.js",
    "manifest.webmanifest",
    "icon.svg",
    "sw.js",
    "session.js",
    "flow.js",
    "pump.js",
    "api.js",
    "capture.js",
    "halt.js",
    "models.js",
    "pairing.js",
    "sharing.js",
    "render.js",
];

/// Serve an allowlisted asset, `Ok(None)` when the path is not an asset.
///
/// # Errors
/// 404 when an allowlisted file is missing from the configured directory.
pub(crate) async fn serve(dir: Option<&Path>, path: &str) -> Result<Option<Response>, ApiError> {
    let (Some(dir), Some(rest)) = (dir, path.strip_prefix("/companion/")) else {
        return Ok(None);
    };
    let file = if rest.is_empty() { "index.html" } else { rest };
    if !FILES.contains(&file) {
        return Ok(None);
    }
    let bytes = tokio::fs::read(dir.join(file))
        .await
        .map_err(|_| ApiError::new(404, "Asset not found"))?;
    Ok(Some((headers(file), bytes).into_response()))
}
