//! Bind native tool execution to the cognition runtime at startup.
use crate::cognition::CognitionRuntime;
use codetether_plugin_verification::PluginRegistry;
use std::sync::Arc;
pub(super) fn install(cognition: &mut CognitionRuntime) -> PluginRegistry {
    let runtime_tools = Arc::new(crate::tool::ToolRegistry::with_defaults());
    let plugin_registry = runtime_tools.plugins().clone();
    cognition.set_tools(runtime_tools);
    plugin_registry
}
