//! Workspace-specific sandbox configuration for Bash execution.

use crate::config::Config;
use serde_json::Value;
use std::path::Path;

pub(super) async fn enabled(default: bool, args: &Value) -> bool {
    let loaded = match crate::tool::network_access::trusted_workspace(args).map(Path::new) {
        Some(workspace) => Config::load_for_workspace(workspace).await,
        None => Config::load().await,
    };
    loaded
        .map(|config| super::from_mode(default, config.effective_sandbox_mode()))
        .unwrap_or(default)
}