//! Load sandbox policy from the invocation's trusted workspace.

use crate::config::Config;
use serde_json::Value;
use std::path::Path;

pub(super) async fn config(args: &Value) -> Config {
    let loaded = match crate::tool::network_access::trusted_workspace(args).map(Path::new) {
        Some(workspace) => Config::load_for_workspace(workspace).await,
        None => Config::load().await,
    };
    loaded.unwrap_or_default()
}