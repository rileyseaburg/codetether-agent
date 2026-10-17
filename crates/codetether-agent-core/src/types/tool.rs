//! Tool catalog metadata and recorded invocations.
use serde::{Deserialize, Serialize};

/// Metadata describing a tool registered with an agent.
///
/// The agent surfaces this alongside the raw tool registry so UIs and remote
/// clients can render descriptions and parameter schemas.
///
/// # Examples
///
/// ```rust
/// use codetether_agent_core::ToolMetadata;
/// let metadata = ToolMetadata { name: "bash".into(), description: "Run shell commands".into(), parameters: serde_json::json!({}) };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolMetadata {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

/// Records a single tool invocation executed during an agent run.
///
/// Sessions store these values so downstream tooling can inspect tool history
/// without re-parsing provider messages.
///
/// # Examples
///
/// ```rust
/// use codetether_agent_core::ToolUse;
/// let use_record = ToolUse { id: "1".into(), name: "bash".into(), input: "{}".into(), output: "ok".into(), success: true };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolUse {
    pub id: String,
    pub name: String,
    pub input: String,
    pub output: String,
    pub success: bool,
}
