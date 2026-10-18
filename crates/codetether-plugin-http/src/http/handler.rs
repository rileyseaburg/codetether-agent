//! HTTP state extraction for plugin discovery.
use super::HttpState;
use crate::PluginListResponse;
use axum::{Json, extract::State};

pub(super) async fn list_plugins<S: HttpState>(State(state): State<S>) -> Json<PluginListResponse> {
    crate::list_plugins(state.plugin_registry(), state.server_fingerprint()).await
}
