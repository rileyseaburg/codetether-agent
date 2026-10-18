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

use crate::{Agent, Host};
use codetether_tool_core::Tool;
use std::sync::Arc;

impl<H: Host> Agent<H> {
    /// Returns a registered tool by name when it exists.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let bash = agent.get_tool("bash");
    /// ```
    pub fn get_tool(&self, name: &str) -> Option<Arc<dyn Tool>> {
        H::tool(&self.tools, name)
    }

    /// Registers a tool with the agent's tool registry.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// agent.register_tool(tool);
    /// ```
    pub fn register_tool(&mut self, tool: Arc<dyn Tool>) {
        H::register(&mut self.tools, tool);
    }

    /// Lists all tool identifiers currently available to the agent.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let ids = agent.list_tools();
    /// ```
    pub fn list_tools(&self) -> Vec<&str> {
        H::names(&self.tools)
    }

    /// Returns whether a tool id is available to the agent.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// assert!(agent.has_tool("bash") || !agent.has_tool("bash"));
    /// ```
    pub fn has_tool(&self, name: &str) -> bool {
        H::tool(&self.tools, name).is_some()
    }
}
