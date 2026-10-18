//! Dynamic tool discovery records and a heartbeat-expiring registry.
//!
//! ToolRegistry retains caller-supplied registrations; RegisteredToolView adds
//! the discovery-only execution flag. HTTP handlers and actual tool execution
//! remain the caller's responsibility.
use std::{collections::HashMap, sync::Arc};
mod cleanup;
pub mod contract;
mod get;
mod heartbeat;
mod list;
mod new;
mod record;
mod register;
mod view;
pub use record::RegisteredTool;
pub use view::RegisteredToolView;

/// Tool registry with TTL-based expiry
#[derive(Clone)]
/// # Examples
/// ```
/// let _registry = codetether_tool_discovery::ToolRegistry::new();
/// ```
pub struct ToolRegistry {
    tools: Arc<tokio::sync::RwLock<HashMap<String, RegisteredTool>>>,
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(test)]
mod view_tests;
