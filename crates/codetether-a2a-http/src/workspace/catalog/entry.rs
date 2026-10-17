//! Workspace catalog entry handling.
use super::*;

pub(super) fn resolve_entry(
    entry: &serde_json::Value,
    roots: &[String],
) -> Option<ResolvedWorkspace> {
    let id = entry["id"].as_str()?.trim();
    let path = entry["path"].as_str()?.trim();
    (!id.is_empty() && !path.is_empty() && is_under_workspace_root(path, roots)).then(|| {
        ResolvedWorkspace {
            id: id.to_string(),
            path: path.to_string(),
        }
    })
}
