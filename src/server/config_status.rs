//! Bind application configuration and package identity to shared status routes.
use crate::config::Config;
use std::sync::Arc;
pub(super) fn router(config: Arc<Config>) -> axum::Router<super::AppState> {
    codetether_server_status::router(config, env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_NAME"))
}
