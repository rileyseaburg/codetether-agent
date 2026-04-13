//! Public agent data types.
//!
//! This module groups the serializable metadata and result types shared by the
//! agent runtime, server, and session layers.
//!
//! # Examples
//!
//! ```ignore
//! let mode = AgentMode::Primary;
//! assert_eq!(mode, AgentMode::Primary);
//! ```

use serde::{Deserialize, Serialize};

/// Describes a registered agent profile and runtime defaults.
///
/// These fields are exposed through the server and UI so callers can inspect
/// the agent catalog and choose an execution mode.
///
/// # Examples
///
/// ```ignore
/// let info = AgentInfo { name: "build".into(), description: None, mode: AgentMode::Primary, native: true, hidden: false, model: None, temperature: None, top_p: None, max_steps: Some(100) };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInfo {
    pub name: String,
    pub description: Option<String>,
    pub mode: AgentMode,
    pub native: bool,
    pub hidden: bool,
    pub model: Option<String>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub max_steps: Option<usize>,
}

/// Categorizes where an agent can be presented or spawned.
///
/// Primary agents are visible entrypoints, subagents are delegated workers, and
/// `All` is used for filters that should include both groups.
///
/// # Examples
///
/// ```ignore
/// match AgentMode::Primary { AgentMode::Primary => (), _ => unreachable!() }
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AgentMode {
    Primary,
    Subagent,
    All,
}

/// Metadata describing a tool registered with an agent.
///
/// The agent surfaces this alongside the raw tool registry so UIs and remote
/// clients can render descriptions and parameter schemas.
///
/// # Examples
///
/// ```ignore
/// let metadata = ToolMetadata { name: "bash".into(), description: "Run shell commands".into(), parameters: serde_json::json!({}) };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolMetadata {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

/// Final response returned from a completed agent run.
///
/// This includes the textual answer along with recorded tool activity and
/// provider usage metadata captured in the session.
///
/// # Examples
///
/// ```ignore
/// let response = AgentResponse { text: String::new(), tool_uses: vec![], usage: Default::default() };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResponse {
    pub text: String,
    pub tool_uses: Vec<ToolUse>,
    pub usage: crate::provider::Usage,
}

/// Records a single tool invocation executed during an agent run.
///
/// Sessions store these values so downstream tooling can inspect tool history
/// without re-parsing provider messages.
///
/// # Examples
///
/// ```ignore
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
