//! Workspace-root resolution for local swarm participants.

use std::path::PathBuf;

pub(crate) fn resolve_root(configured: Option<&str>) -> PathBuf {
    configured
        .map(PathBuf::from)
        .or_else(|| std::env::current_dir().ok())
        .unwrap_or_else(|| PathBuf::from("."))
}
