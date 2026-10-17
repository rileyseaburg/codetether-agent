//! Workspace catalog entries handling.

pub(crate) fn workspace_entries(data: serde_json::Value) -> Vec<serde_json::Value> {
    data.as_array()
        .cloned()
        .or_else(|| data["workspaces"].as_array().cloned())
        .or_else(|| data["codebases"].as_array().cloned())
        .unwrap_or_default()
}
