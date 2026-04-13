//! Core agent state and construction.
//!
//! This module defines the main `Agent` struct and its constructor. Execution,
//! tool dispatch, and swarm integration live in separate submodules.
//!
//! # Examples
//!
//! ```ignore
//! let agent = Agent::new(info, provider, tools, "system".into());
//! ```

use crate::config::PermissionAction;
use crate::provider::Provider;
use crate::tool::ToolRegistry;
use std::collections::HashMap;
use std::sync::Arc;

use super::{AgentInfo, ToolMetadata};

/// Stateful runtime for executing prompts with tools and a provider.
///
/// An `Agent` owns the provider, tool registry, and permission metadata used
/// while serving a session.
///
/// # Examples
///
/// ```ignore
/// let agent = Agent::new(info, provider, tools, "system".into());
/// ```
pub struct Agent {
    /// Static metadata describing the agent.
    pub info: AgentInfo,
    /// Provider used for model completions.
    pub provider: Arc<dyn Provider>,
    /// Tool registry available to the agent.
    pub tools: ToolRegistry,
    /// Per-tool permission policy for audit and enforcement.
    pub permissions: HashMap<String, PermissionAction>,
    /// Metadata for registered tools.
    pub metadata: HashMap<String, ToolMetadata>,
    pub(super) system_prompt: String,
}

impl Agent {
    /// Constructs a new agent from provider, tool, and prompt state.
    ///
    /// The agent starts with empty permission and metadata maps so callers can
    /// customize them after construction.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let agent = Agent::new(info, provider, tools, "system".into());
    /// ```
    pub fn new(
        info: AgentInfo,
        provider: Arc<dyn Provider>,
        tools: ToolRegistry,
        system_prompt: String,
    ) -> Self {
        Self {
            info,
            provider,
            tools,
            permissions: HashMap::new(),
            metadata: HashMap::new(),
            system_prompt,
        }
    }
}
