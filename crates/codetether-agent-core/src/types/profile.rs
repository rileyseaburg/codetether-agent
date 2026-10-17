//! Agent profile metadata and presentation mode.
use serde::{Deserialize, Serialize};

/// Describes a registered agent profile and runtime defaults.
///
/// These fields are exposed through the server and UI so callers can inspect
/// the agent catalog and choose an execution mode.
///
/// # Examples
///
/// ```rust
/// use codetether_agent_core::AgentInfo;
/// use codetether_agent_core::AgentMode;
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
/// ```rust
/// use codetether_agent_core::AgentMode;
/// match AgentMode::Primary { AgentMode::Primary => (), AgentMode::Subagent => (), AgentMode::All => () }
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AgentMode {
    Primary,
    Subagent,
    All,
}
