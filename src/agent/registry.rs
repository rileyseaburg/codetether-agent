//! Agent registry implementation.
//!
//! This module stores the catalog of available agents and offers filtered views
//! for UI and server consumers.
//!
//! # Examples
//!
//! ```ignore
//! let registry = AgentRegistry::with_builtins();
//! assert!(!registry.list().is_empty());
//! ```

use super::{AgentInfo, AgentMode, builtin};
use std::collections::HashMap;

/// Registry of known agent profiles.
///
/// The registry is a simple in-memory catalog used by the server, tests, and
/// UI layers to enumerate and select agents.
///
/// # Examples
///
/// ```ignore
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
    /// ```ignore
    /// let registry = AgentRegistry::new();
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
    /// ```ignore
    /// registry.register(info);
    /// ```
    pub fn register(&mut self, info: AgentInfo) {
        self.agents.insert(info.name.clone(), info);
    }

    /// Returns an agent profile by name when present.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let build = registry.get("build");
    /// ```
    #[allow(dead_code)]
    pub fn get(&self, name: &str) -> Option<&AgentInfo> {
        self.agents.get(name)
    }

    /// Lists every registered agent profile.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let all = registry.list();
    /// ```
    pub fn list(&self) -> Vec<&AgentInfo> {
        self.agents.values().collect()
    }

    /// Lists visible primary agents for UI presentation.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let primary = registry.list_primary();
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
    /// ```ignore
    /// let registry = AgentRegistry::with_builtins();
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
