//! Trusted workspace selection for persistent command execution.

use serde_json::Value;
use std::path::{Path, PathBuf};

pub(super) fn resolve(args: &Value, fallback: Option<&Path>) -> Option<PathBuf> {
    crate::tool::network_access::trusted_workspace(args)
        .map(PathBuf::from)
        .or_else(|| fallback.map(Path::to_path_buf))
}