//! Built-in agent catalog entries.
//!
//! This module defines the static metadata for the built-in `build`, `plan`,
//! and `explore` agents.
//!
//! # Examples
//!
//! ```ignore
//! let info = build_agent();
//! assert_eq!(info.name, "build");
//! ```

use crate::agent::{AgentInfo, AgentMode};

/// Returns the default full-access build agent profile.
///
/// # Examples
///
/// ```ignore
/// let info = build_agent();
/// assert_eq!(info.mode, AgentMode::Primary);
/// ```
pub fn build_agent() -> AgentInfo {
    AgentInfo {
        name: "build".to_string(),
        description: Some("Full access agent for development work".to_string()),
        mode: AgentMode::Primary,
        native: true,
        hidden: false,
        model: None,
        temperature: None,
        top_p: None,
        max_steps: Some(100),
    }
}

/// Returns the read-only planning agent profile.
///
/// # Examples
///
/// ```ignore
/// let info = plan_agent();
/// assert_eq!(info.name, "plan");
/// ```
pub fn plan_agent() -> AgentInfo {
    AgentInfo {
        name: "plan".to_string(),
        description: Some("Read-only agent for analysis and code exploration".to_string()),
        mode: AgentMode::Primary,
        native: true,
        hidden: false,
        model: None,
        temperature: None,
        top_p: None,
        max_steps: Some(50),
    }
}

/// Returns the lightweight explore agent profile.
///
/// # Examples
///
/// ```ignore
/// let info = explore_agent();
/// assert_eq!(info.mode, AgentMode::Subagent);
/// ```
pub fn explore_agent() -> AgentInfo {
    AgentInfo {
        name: "explore".to_string(),
        description: Some("Fast agent for exploring codebases".to_string()),
        mode: AgentMode::Subagent,
        native: true,
        hidden: false,
        model: None,
        temperature: None,
        top_p: None,
        max_steps: Some(20),
    }
}
