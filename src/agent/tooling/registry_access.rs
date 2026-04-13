//! Tool registry accessors for agents.
//!
//! This module keeps simple tool-registry convenience methods out of the main
//! dispatch file.
//!
//! # Examples
//!
//! ```ignore
//! let ids = agent.list_tools();
//! ```

use crate::agent::Agent;
use crate::tool::Tool;
use std::sync::Arc;

impl Agent {
    /// Returns a registered tool by name when it exists.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let bash = agent.get_tool("bash");
    /// ```
    pub fn get_tool(&self, name: &str) -> Option<Arc<dyn Tool>> {
        self.tools.get(name)
    }

    /// Registers a tool with the agent's tool registry.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// agent.register_tool(tool);
    /// ```
    pub fn register_tool(&mut self, tool: Arc<dyn Tool>) {
        self.tools.register(tool);
    }

    /// Lists all tool identifiers currently available to the agent.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let ids = agent.list_tools();
    /// ```
    pub fn list_tools(&self) -> Vec<&str> {
        self.tools.list()
    }

    /// Returns whether a tool id is available to the agent.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// assert!(agent.has_tool("bash") || !agent.has_tool("bash"));
    /// ```
    pub fn has_tool(&self, name: &str) -> bool {
        self.tools.get(name).is_some()
    }
}
