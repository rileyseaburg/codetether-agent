//! Agent registry implementation.
//!
//! This module stores the catalog of available agents and offers filtered views
//! for UI and server consumers.
//!
//! # Examples
//!
//! ```rust
//! use codetether_agent_core::AgentRegistry;
//! let registry = AgentRegistry::with_builtins();
//! assert!(!registry.list().is_empty());
//! ```

use super::definitions as builtin;
use crate::{AgentInfo, AgentMode};
use std::collections::HashMap;

/// Registry of known agent profiles.
///
/// The registry is a simple in-memory catalog used by the server, tests, and
/// UI layers to enumerate and select agents.
///
/// # Examples
///
/// ```rust
/// use codetether_agent_core::AgentRegistry;
/// let registry = AgentRegistry::new();
/// ```
pub struct AgentRegistry {
    agents: HashMap<String, AgentInfo>,
}

impl AgentRegistry {
    /// Creates an empty registry with no built-in agents.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent_core::AgentRegistry;
    /// let registry = AgentRegistry::new();
    ///     
    /// ```
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
        }
    }

    /// Registers an agent profile in the catalog.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent_core::AgentRegistry;
    /// let mut registry = AgentRegistry::new();
    /// let info = codetether_agent_core::builtin::build_agent();
    /// registry.register(info);
    ///     
    /// ```
    pub fn register(&mut self, info: AgentInfo) {
        self.agents.insert(info.name.clone(), info);
    }

    /// Returns an agent profile by name when present.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent_core::AgentRegistry;
    /// let registry = AgentRegistry::with_builtins();
    /// let build = registry.get("build");
    ///     
    /// ```
    #[allow(dead_code)]
    pub fn get(&self, name: &str) -> Option<&AgentInfo> {
        self.agents.get(name)
    }

    /// Lists every registered agent profile.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent_core::AgentRegistry;
    /// let registry = AgentRegistry::with_builtins();
    /// let all = registry.list();
    ///     
    /// ```
    pub fn list(&self) -> Vec<&AgentInfo> {
        self.agents.values().collect()
    }

    /// Lists visible primary agents for UI presentation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent_core::AgentRegistry;
    /// let registry = AgentRegistry::with_builtins();
    /// let primary = registry.list_primary();
    ///     
    /// ```
    #[allow(dead_code)]
    pub fn list_primary(&self) -> Vec<&AgentInfo> {
        self.agents
            .values()
            .filter(|agent| agent.mode == AgentMode::Primary && !agent.hidden)
            .collect()
    }

    /// Creates a registry pre-populated with built-in agents.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent_core::AgentRegistry;
    /// let registry = AgentRegistry::with_builtins();
    ///     
    /// ```
    pub fn with_builtins() -> Self {
        let mut registry = Self::new();
        registry.register(builtin::build_agent());
        registry.register(builtin::plan_agent());
        registry.register(builtin::explore_agent());
        registry
    }
}

impl Default for AgentRegistry {
    fn default() -> Self {
        Self::with_builtins()
    }
}
