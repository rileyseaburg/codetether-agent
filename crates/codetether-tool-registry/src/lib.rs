//! Tool collection, definition profiles, compatibility aliases and plugin state.
//!
//! Construct [ToolRegistry], register application tools, then query tools or
//! their provider-facing definitions. Application factories own the default
//! tool set and runtime policy; this crate owns registry storage operations.
use codetether_plugin_verification::PluginRegistry;
use codetether_tool_core::Tool;
use std::{collections::HashMap, sync::Arc};
mod alias;
mod collection;
mod create;
mod definitions;
/// Tool map and signed-plugin registry owned by one application tool set.
///
/// # Examples
/// ```no_run
/// let registry = codetether_tool_registry::ToolRegistry::new();
/// assert!(registry.is_empty());
/// ```
pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn Tool>>,
    plugin_registry: PluginRegistry,
}

impl std::fmt::Debug for ToolRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ToolRegistry")
            .field("tools", &self.tools.keys().collect::<Vec<_>>())
            .finish()
    }
}
