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

use crate::Host;
use codetether_config::PermissionAction;
use codetether_provider_core::Provider;
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
/// ```rust
/// # fn example<H: codetether_agent_runtime::Host>(info: codetether_agent_core::AgentInfo, provider: std::sync::Arc<dyn codetether_provider_core::Provider>, tools: H::Tools) {
/// let agent = codetether_agent_runtime::Agent::<H>::new(info, provider, tools, "system".into());
/// assert!(agent.permissions.is_empty());
/// # }
/// ```
pub struct Agent<H: Host> {
    /// Static metadata describing the agent.
    pub info: AgentInfo,
    /// Provider used for model completions.
    pub provider: Arc<dyn Provider>,
    /// Tool registry available to the agent.
    pub tools: H::Tools,
    /// Per-tool permission policy for audit and enforcement.
    pub permissions: HashMap<String, PermissionAction>,
    /// Metadata for registered tools.
    pub metadata: HashMap<String, ToolMetadata>,
    pub(super) system_prompt: String,
}

impl<H: Host> Agent<H> {
    /// Constructs a new agent from provider, tool, and prompt state.
    ///
    /// The agent starts with empty permission and metadata maps so callers can
    /// customize them after construction.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn example<H: codetether_agent_runtime::Host>(info: codetether_agent_core::AgentInfo, provider: std::sync::Arc<dyn codetether_provider_core::Provider>, tools: H::Tools) {
    /// let agent = codetether_agent_runtime::Agent::<H>::new(info, provider, tools, "system".into());
    /// assert!(agent.permissions.is_empty());
    /// # }
    /// ```
    pub fn new(
        info: AgentInfo,
        provider: Arc<dyn Provider>,
        tools: H::Tools,
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
