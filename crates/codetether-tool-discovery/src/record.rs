//! Registered tool metadata and heartbeat expiry timestamps.
use serde::{Deserialize, Serialize};

/// A registered tool with TTL tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
/// # Examples
/// ```
/// let now = chrono::Utc::now();
/// let tool = codetether_tool_discovery::RegisteredTool {
///     id: "demo".into(), name: "Demo".into(), description: "demo tool".into(),
///     version: "1".into(), endpoint: "http://localhost/tool".into(),
///     capabilities: vec![], parameters: serde_json::json!({}),
///     execution_mode: "discovery_only".into(), registered_at: now,
///     last_heartbeat: now, expires_at: now,
/// };
/// assert_eq!(tool.id, "demo");
/// ```
pub struct RegisteredTool {
    /// Unique registration identifier.
    pub id: String,
    /// Human-readable tool name.
    pub name: String,
    /// Tool capability description.
    pub description: String,
    /// Tool version string.
    pub version: String,
    /// Registered service endpoint.
    pub endpoint: String,
    /// Advertised capability names.
    pub capabilities: Vec<String>,
    /// JSON parameter schema.
    pub parameters: serde_json::Value,
    /// Discovery or execution mode label.
    pub execution_mode: String,
    /// Initial registration timestamp.
    pub registered_at: chrono::DateTime<chrono::Utc>,
    /// Most recent heartbeat timestamp.
    pub last_heartbeat: chrono::DateTime<chrono::Utc>,
    /// Expiry timestamp used by listing and cleanup.
    pub expires_at: chrono::DateTime<chrono::Utc>,
}
