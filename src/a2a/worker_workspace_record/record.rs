use serde::Deserialize;

/// Remote workspace metadata returned by the A2A server.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::a2a::worker_workspace_record::RegisteredWorkspaceRecord;
///
/// let record: RegisteredWorkspaceRecord = serde_json::from_value(serde_json::json!({
///     "id": "workspace-1",
///     "name": "example",
///     "path": "/tmp/example",
///     "description": "Example workspace",
///     "agent_config": {},
///     "git_url": "https://example.com/repo.git",
///     "git_branch": "main"
/// }))
/// .expect("workspace record");
///
/// assert_eq!(record.id, "workspace-1");
/// assert_eq!(record.path, "/tmp/example");
/// ```
#[derive(Debug, Deserialize)]
pub struct RegisteredWorkspaceRecord {
    pub id: String,
    pub name: String,
    pub path: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub agent_config: serde_json::Map<String, serde_json::Value>,
    pub git_url: Option<String>,
    pub git_branch: Option<String>,
}
