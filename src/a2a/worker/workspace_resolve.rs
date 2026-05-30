//! Resolve workspace IDs from the control plane that match worker local paths.
use anyhow::Result;
use reqwest::Client;
/// A resolved workspace linking a server-side ID to a local path.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ResolvedWorkspace {
    /// Server-side workspace ID.
    pub id: String,
    /// Local filesystem path.
    pub path: String,
}
/// Queries the A2A server for registered workspaces under configured roots.
pub async fn resolve_workspace_ids(
    client: &Client,
    server: &str,
    token: &Option<String>,
    workspace_roots: &[String],
) -> Result<Vec<ResolvedWorkspace>> {
    let mut req = client.get(format!(
        "{}/v1/agent/workspaces",
        server.trim_end_matches('/')
    ));
    if let Some(t) = token {
        req = req.bearer_auth(t);
    }
    let res = req.send().await?;
    if !res.status().is_success() {
        tracing::debug!(status = %res.status(), "Workspace ID resolution: server returned non-success");
        return Ok(Vec::new());
    }
    let data: serde_json::Value = res.json().await?;
    Ok(workspace_entries(data)
        .into_iter()
        .filter_map(|entry| resolve_entry(&entry, workspace_roots))
        .collect())
}
fn workspace_entries(data: serde_json::Value) -> Vec<serde_json::Value> {
    data.as_array()
        .cloned()
        .or_else(|| data["workspaces"].as_array().cloned())
        .or_else(|| data["codebases"].as_array().cloned())
        .unwrap_or_default()
}
fn resolve_entry(entry: &serde_json::Value, roots: &[String]) -> Option<ResolvedWorkspace> {
    let id = entry["id"].as_str()?.trim();
    let path = entry["path"].as_str()?.trim();
    (!id.is_empty() && !path.is_empty() && is_under_workspace_root(path, roots)).then(|| {
        ResolvedWorkspace {
            id: id.to_string(),
            path: path.to_string(),
        }
    })
}
fn is_under_workspace_root(path: &str, roots: &[String]) -> bool {
    let path = std::path::Path::new(path);
    roots
        .iter()
        .map(std::path::Path::new)
        .any(|root| path == root || path.starts_with(root))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn path_under_root_matches() {
        let roots = vec!["/workspace".to_string()];
        assert!(is_under_workspace_root("/workspace", &roots));
        assert!(is_under_workspace_root("/workspace/spotlessbinco", &roots));
        assert!(!is_under_workspace_root("/other/path", &roots));
    }
    #[test]
    fn empty_roots_match_nothing() {
        assert!(!is_under_workspace_root("/workspace/foo", &[]));
    }
}
