//! Application package metadata and current executable content hash.
use crate::state::StatusState;
use axum::{Extension, Json};
use codetether_plugin_verification::hash_file;
use serde::Serialize;

/// Version response body.
#[derive(Serialize)]
pub(super) struct VersionInfo {
    version: &'static str,
    name: &'static str,
    binary_hash: Option<String>,
}

/// Return package version and binary hash metadata.
pub(super) async fn get_version(Extension(state): Extension<StatusState>) -> Json<VersionInfo> {
    let binary_hash = std::env::current_exe()
        .ok()
        .and_then(|p| hash_file(&p).ok());
    Json(VersionInfo {
        version: state.version,
        name: state.name,
        binary_hash,
    })
}
