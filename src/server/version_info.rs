//! Version endpoint handler.

use crate::tool::hash_file;
use axum::Json;
use serde::Serialize;

/// Version response body.
#[derive(Serialize)]
pub(super) struct VersionInfo {
    version: &'static str,
    name: &'static str,
    binary_hash: Option<String>,
}

/// Return package version and binary hash metadata.
pub(super) async fn get_version() -> Json<VersionInfo> {
    let binary_hash = std::env::current_exe()
        .ok()
        .and_then(|p| hash_file(&p).ok());
    Json(VersionInfo {
        version: env!("CARGO_PKG_VERSION"),
        name: env!("CARGO_PKG_NAME"),
        binary_hash,
    })
}
