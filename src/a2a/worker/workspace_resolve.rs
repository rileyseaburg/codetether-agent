//! Resolve workspace IDs from the control plane that match the worker's local paths.
//!
//! When a worker starts with `--workspaces /workspace`, it only knows filesystem
//! paths. The A2A server routes tasks by **workspace ID** (e.g. `spotlessbinco`),
//! so the worker must query the server to discover which IDs map to paths that
//! exist (or could exist) under its configured directories.

use anyhow::Result;
use reqwest::Client;

/// A resolved workspace linking a server-side ID to a local path.
#[derive(Debug, Clone)]
pub struct ResolvedWorkspace {
    /// Server-side workspace ID (e.g. "spotlessbinco").
    pub id: String,
    /// Local filesystem path (e.g. "/workspace/spotlessbinco").
    pub path: String,
}

/// Queries the A2A server for all registered workspaces and resolves which
/// ones correspond to paths under the worker's configured workspace directories.
///
/// Returns a list of `(workspace_id, local_path)` pairs. Paths are considered
/// matching if they are **subdirectories of** any configured workspace root,
/// regardless of whether the path currently exists on disk.
///
/// # Examples
///
/// ```rust,no_run
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// use codetether_agent::a2a::worker::workspace_resolve::resolve_workspace_ids;
/// use reqwest::Client;
///
/// let client = Client::new();
/// let matches = resolve_workspace_ids(
///     &client,
///     "http://codetether-a2a-server:8000",
///     &None,
///     &["/workspace".to_string()],
/// )
/// .await
/// .expect("resolve");
///
/// for ws in &matches {
///     println!("id={} path={}", ws.id, ws.path);
/// }
/// # });
/// ```
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
        tracing::debug!(
            status = %res.status(),
            "Workspace ID resolution: server returned non-success"
        );
        return Ok(Vec::new());
    }

    let data: serde_json::Value = res.json().await?;
    let entries = data
        .as_array()
        .cloned()
        .or_else(|| data["workspaces"].as_array().cloned())
        .or_else(|| data["codebases"].as_array().cloned())
        .unwrap_or_default();

    let mut resolved = Vec::new();
    for entry in &entries {
        let id = match entry["id"].as_str() {
            Some(id) if !id.trim().is_empty() => id.trim().to_string(),
            _ => continue,
        };
        let path = match entry["path"].as_str() {
            Some(p) if !p.trim().is_empty() => p.trim().to_string(),
            _ => continue,
        };

        // Check if the workspace path falls under any of our configured roots.
        if is_under_workspace_root(&path, workspace_roots) {
            resolved.push(ResolvedWorkspace { id, path });
        }
    }

    Ok(resolved)
}

/// Returns true if `path` is equal to or a subdirectory of any workspace root.
fn is_under_workspace_root(path: &str, roots: &[String]) -> bool {
    let path = std::path::Path::new(path);
    roots.iter().any(|root| {
        let root = std::path::Path::new(root);
        path == root || path.starts_with(root)
    })
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

    #[test]
    fn exact_root_match() {
        let roots = vec!["/workspace".to_string()];
        assert!(is_under_workspace_root("/workspace", &roots));
    }
}
